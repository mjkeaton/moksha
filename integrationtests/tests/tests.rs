#![allow(unused_imports)]
use moksha_wallet::client::reqwest::HttpClient;
use moksha_wallet::client::LegacyClient;
use moksha_wallet::localstore::sqlite::SqliteLocalStore;
use moksha_wallet::wallet::WalletBuilder;
use mokshamint::lightning::{LightningType, LnbitsLightningSettings};
use mokshamint::mint::Mint;
use reqwest::Url;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use testcontainers::core::WaitFor;
use testcontainers::{clients, Image};
use tokio::runtime::Runtime;
use tokio::time::{sleep_until, Instant};

#[test]
#[cfg(feature = "integration-tests")]
pub fn test_integration() -> anyhow::Result<()> {
    let docker = clients::Cli::default();
    let node = docker.run(Postgres::default());
    let host_port = node.get_host_port_ipv4(5432);

    // start lnbits
    let _lnbits_thread = thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async {
            let _ = lnbitsmock::run_server(6100).await;
        });
    });

    // Create a channel to signal when the server has started
    let _server_thread = thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async {
            let mint = Mint::builder()
                .with_private_key("my_private_key".to_string())
                .with_db(
                    format!(
                        "postgres://postgres:postgres@127.0.0.1:{}/moksha-mint",
                        host_port
                    )
                    .to_owned(),
                )
                .with_lightning(LightningType::Lnbits(LnbitsLightningSettings::new(
                    "my_admin_key",
                    "http://127.0.0.1:6100",
                )))
                .with_fee(0.0, 0)
                .build();

            let result = mokshamint::server::run_server(
                mint.await.expect("Can not connect to lightning backend"),
                "127.0.0.1:8686".parse().expect("invalid address"),
                None,
                None,
            )
            .await;
            assert!(result.is_ok());
        });
    });

    // Wait for the server to start
    std::thread::sleep(std::time::Duration::from_millis(800));

    let client = HttpClient::default();
    let mint_url = Url::parse("http://127.0.0.1:8686")?;
    let rt = Runtime::new()?;
    rt.block_on(async move {
        let keys = client.get_mint_keys(&mint_url).await;
        assert!(keys.is_ok());

        let keysets = client.get_mint_keysets(&mint_url).await;
        assert!(keysets.is_ok());

        // create wallet
        let tmp = tempfile::tempdir().expect("Could not create tmp dir for wallet");
        let tmp_dir = tmp
            .path()
            .to_str()
            .expect("Could not create tmp dir for wallet");

        let localstore = SqliteLocalStore::with_path(format!("{tmp_dir}/test_wallet.db"))
            .await
            .expect("Could not create localstore");

        let wallet = WalletBuilder::default()
            .with_client(client)
            .with_localstore(localstore)
            .with_mint_url(mint_url)
            .build()
            .await
            .expect("Could not create wallet");

        // get initial balance
        let balance = wallet.get_balance().await.expect("Could not get balance");
        assert_eq!(0, balance, "Initial balance should be 0");

        // mint some tokens
        let mint_amount = 6_000;
        let payment_request = wallet.get_mint_payment_request(mint_amount).await.unwrap();
        let hash = payment_request.clone().hash;

        sleep_until(Instant::now() + Duration::from_millis(1_000)).await;
        let mint_result = wallet
            .mint_tokens(mint_amount.into(), hash.clone())
            .await
            .unwrap();
        assert_eq!(6_000, mint_result.total_amount());

        let balance = wallet.get_balance().await.expect("Could not get balance");
        assert_eq!(6_000, balance);

        // pay ln-invoice
        let invoice_1000 = read_fixture("invoice_1000.txt").unwrap();
        let result_pay_invoice = wallet.pay_invoice(invoice_1000).await;
        assert!(result_pay_invoice.is_ok());
        let balance = wallet.get_balance().await.expect("Could not get balance");
        assert_eq!(5_000, balance);

        // receive 10 sats
        let token_10: moksha_core::token::TokenV3 =
            read_fixture("token_10.cashu").unwrap().try_into().unwrap();
        let result_receive = wallet.receive_tokens(&token_10).await;
        assert!(result_receive.is_ok());
        let balance = wallet.get_balance().await.expect("Could not get balance");
        assert_eq!(5_010, balance);

        // send 10 tokens
        let result_send = wallet.send_tokens(10).await;
        assert!(result_send.is_ok());
        assert_eq!(10, result_send.unwrap().total_amount());
        let balance = wallet.get_balance().await.expect("Could not get balance");
        assert_eq!(5_000, balance);
    });

    Ok(())
}

#[cfg(feature = "integration-tests")]
fn read_fixture(name: &str) -> anyhow::Result<String> {
    let base_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let raw_token = std::fs::read_to_string(format!("{base_dir}/tests/fixtures/{name}"))?;
    Ok(raw_token.trim().to_string())
}

#[derive(Debug)]
pub struct Postgres {
    env_vars: HashMap<String, String>,
}

impl Default for Postgres {
    fn default() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert("POSTGRES_DB".to_owned(), "postgres".to_owned());
        env_vars.insert("POSTGRES_HOST_AUTH_METHOD".into(), "trust".into());
        env_vars.insert("POSTGRES_DB".into(), "moksha-mint".into());

        Self { env_vars }
    }
}

impl Image for Postgres {
    type Args = ();

    fn name(&self) -> String {
        "postgres".to_owned()
    }

    fn tag(&self) -> String {
        "15.3".to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        )]
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.env_vars.iter())
    }
}
