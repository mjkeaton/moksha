use crate::routes::btconchain::{
    get_melt_btconchain, get_melt_quote_btconchain, get_mint_quote_btconchain,
    post_melt_btconchain, post_melt_quote_btconchain, post_mint_btconchain,
    post_mint_quote_btconchain,
};
use crate::routes::default::{
    check_bitcredit_quote, get_info, get_keys, get_keys_by_id, get_keys_old, get_keysets,
    get_keysets_by_id, get_keysets_old, get_melt_quote_bolt11, get_mint_quote_bitcredit,
    get_mint_quote_bolt11, mjk_get_info, mjk_get_keys, mjk_get_keys_by_id, mjk_get_keysets,
    mjk_post_swap, post_melt_bolt11, post_melt_quote_bolt11, post_mint_bitcredit, post_mint_bolt11,
    post_mint_quote_bitcredit, post_mint_quote_bolt11, post_request_to_mint_bitcredit, post_swap,
};
use axum::extract::Request;
use axum::http::{HeaderName, HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::routing::{get, get_service, post};
use axum::{middleware, Router};

use moksha_core::keyset::{Keyset, Keysets};
use moksha_core::proof::Proofs;
use moksha_core::proof::{P2SHScript, Proof};

use utoipa_swagger_ui::SwaggerUi;

use crate::mint::Mint;

use moksha_core::blind::BlindedMessage;
use moksha_core::blind::BlindedSignature;
use moksha_core::primitives::{
    CheckBitcreditQuoteResponse, CurrencyUnit, GetMeltBtcOnchainResponse, KeyResponse,
    KeysResponse, MintInfoResponse, Nut10, Nut11, Nut12, Nut17, Nut18, Nut4, Nut5, Nut7, Nut8,
    Nut9, Nuts, PaymentMethod, PostMeltBolt11Request, PostMeltBolt11Response,
    PostMeltQuoteBolt11Request, PostMeltQuoteBolt11Response, PostMeltQuoteBtcOnchainRequest,
    PostMeltQuoteBtcOnchainResponse, PostMintBitcreditRequest, PostMintBitcreditResponse,
    PostMintBolt11Request, PostMintBolt11Response, PostMintQuoteBitcreditRequest,
    PostMintQuoteBitcreditResponse, PostMintQuoteBolt11Request, PostMintQuoteBolt11Response,
    PostMintQuoteBtcOnchainRequest, PostMintQuoteBtcOnchainResponse,
    PostRequestToMintBitcreditRequest, PostRequestToMintBitcreditResponse, PostSwapRequest,
    PostSwapResponse,
};

use tower_http::services::ServeDir;

use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use utoipa::OpenApi;

pub async fn run_server(mint: Mint) -> anyhow::Result<()> {
    if let Some(ref buildtime) = mint.build_params.build_time {
        info!("build time: {}", buildtime);
    }
    if let Some(ref commithash) = mint.build_params.commit_hash {
        info!("git commit-hash: {}", commithash);
    }
    if let Some(ref serve_wallet_path) = mint.config.server.serve_wallet_path {
        info!("serving wallet from path: {:?}", serve_wallet_path);
    }
    info!("listening on: {}", &mint.config.server.host_port);
    info!("mint-info: {:?}", mint.config.info);
    info!("lightning fee-reserve: {:?}", mint.config.lightning_fee);
    info!("lightning-backend: {}", mint.lightning_type);

    if let Some(ref onchain) = mint.config.btconchain_backend {
        info!("onchain-type: {:?}", onchain.onchain_type);
        info!(
            "btconchain-min-confirmations: {}",
            onchain.min_confirmations
        );
        info!("btconchain-min-amount: {}", onchain.min_amount);
        info!("btconchain-max-amount: {}", onchain.max_amount);
    } else {
        info!("btconchain-backend is not configured");
    }

    info!("tracing jaeger-endpoint: {:?}", mint.config.tracing);

    let listener = tokio::net::TcpListener::bind(&mint.config.server.host_port).await?;

    axum::serve(
        listener,
        app(mint)
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_headers(Any)
                    .allow_methods(Any)
                    .expose_headers(Any),
            )
            .into_make_service(),
    )
    .await?;

    Ok(())
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::default::get_keys,
        crate::routes::default::get_keys_by_id,
        crate::routes::default::get_keysets,
        crate::routes::default::get_keysets_by_id,
        crate::routes::default::post_mint_bolt11,
        crate::routes::default::post_mint_bitcredit,
        crate::routes::default::post_mint_quote_bolt11,
        crate::routes::default::post_mint_quote_bitcredit,
        crate::routes::default::post_request_to_mint_bitcredit,
        crate::routes::default::get_mint_quote_bolt11,
        crate::routes::default::get_mint_quote_bitcredit,
        crate::routes::default::post_melt_bolt11,
        crate::routes::default::post_melt_quote_bolt11,
        crate::routes::default::get_melt_quote_bolt11,
        crate::routes::default::post_swap,
        crate::routes::default::get_info,
        get_health,
        crate::routes::btconchain::post_mint_quote_btconchain,
        crate::routes::btconchain::get_mint_quote_btconchain,
        crate::routes::btconchain::post_mint_btconchain,
        crate::routes::btconchain::post_melt_quote_btconchain,
        crate::routes::btconchain::get_melt_quote_btconchain,
        crate::routes::btconchain::post_melt_btconchain,
        crate::routes::btconchain::get_melt_btconchain
    ),
    components(schemas(
        MintInfoResponse,
        Nuts,
        Nut4,
        Nut5,
        Nut7,
        Nut8,
        Nut9,
        Nut10,
        Nut11,
        Nut12,
        CurrencyUnit,
        PaymentMethod,
        KeysResponse,
        KeyResponse,
        Keysets,
        Keyset,
        BlindedMessage,
        BlindedSignature,
        Proof,
        Proofs,
        PostMintQuoteBitcreditRequest,
        PostMintQuoteBitcreditResponse,
        PostRequestToMintBitcreditRequest,
        CheckBitcreditQuoteResponse,
        PostRequestToMintBitcreditResponse,
        PostMintQuoteBolt11Request,
        PostMintQuoteBolt11Response,
        PostMeltQuoteBolt11Request,
        PostMeltQuoteBolt11Response,
        PostMeltBolt11Request,
        PostMeltBolt11Response,
        PostMintBolt11Request,
        PostMintBitcreditRequest,
        PostMintBolt11Response,
        PostMintBitcreditResponse,
        PostSwapRequest,
        PostSwapResponse,
        P2SHScript,
        Nut17,
        Nut18,
        PostMintQuoteBtcOnchainRequest,
        PostMintQuoteBtcOnchainResponse,
        PostMeltQuoteBtcOnchainRequest,
        PostMeltQuoteBtcOnchainResponse,
        GetMeltBtcOnchainResponse
    ))
)]
struct ApiDoc;

fn app(mint: Mint) -> Router {
    let default_routes = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        /* mjk */
        .route("/:id/:unit/v1/info", get(mjk_get_info))
        .route("/:id/:unit/v1/keysets", get(mjk_get_keysets))
        .route("/:id/:unit/v1/keys", get(mjk_get_keys))
        .route("/:id/:unit/v1/keys/:id", get(mjk_get_keys_by_id))
        .route("/:id/:unit/v1/swap", post(mjk_post_swap))
        /* mjk - end */
        .route("/v1/keys/:unit", get(get_keys))
        .route("/v1/keys", get(get_keys_old))
        .route("/v1/keys/:id/:unit", get(get_keys_by_id))
        .route("/v1/keysets/:unit", get(get_keysets))
        .route("/v1/keysets", get(get_keysets_old))
        .route("/v1/keysets/:unit/:id", get(get_keysets_by_id))
        .route("/v1/mint/quote/bolt11", post(post_mint_quote_bolt11))
        .route("/v1/mint/quote/bitcredit", post(post_mint_quote_bitcredit))
        .route(
            "/v1/mint/request/bitcredit",
            post(post_request_to_mint_bitcredit),
        )
        .route(
            "/v1/quote/bitcredit/check/:bill_id/:node_id",
            get(check_bitcredit_quote),
        )
        .route("/v1/mint/quote/bolt11/:quote", get(get_mint_quote_bolt11))
        .route(
            "/v1/mint/quote/bitcredit/:quote",
            get(get_mint_quote_bitcredit),
        )
        .route("/v1/mint/bolt11", post(post_mint_bolt11))
        .route("/v1/mint/bitcredit", post(post_mint_bitcredit))
        .route("/v1/melt/quote/bolt11", post(post_melt_quote_bolt11))
        .route("/v1/melt/quote/bolt11/:quote", get(get_melt_quote_bolt11))
        .route("/v1/melt/bolt11", post(post_melt_bolt11))
        .route("/v1/swap", post(post_swap))
        .route("/v1/info", get(get_info));

    let btconchain_routes = if mint.onchain.is_some() {
        Router::new()
            .route(
                "/v1/mint/quote/btconchain",
                post(post_mint_quote_btconchain),
            )
            .route(
                "/v1/mint/quote/btconchain/:quote",
                get(get_mint_quote_btconchain),
            )
            .route("/v1/mint/btconchain", post(post_mint_btconchain))
            .route(
                "/v1/melt/quote/btconchain",
                post(post_melt_quote_btconchain),
            )
            .route(
                "/v1/melt/quote/btconchain/:quote",
                get(get_melt_quote_btconchain),
            )
            .route("/v1/melt/btconchain", post(post_melt_btconchain))
            .route("/v1/melt/btconchain/:txid", get(get_melt_btconchain))
    } else {
        Router::new()
    };

    let general_routes = Router::new().route("/health", get(get_health));

    let server_config = mint.config.server.clone();
    let prefix = server_config.api_prefix.unwrap_or_else(|| "".to_owned());

    let router = Router::new()
        .nest(&prefix, default_routes)
        .nest(&prefix, btconchain_routes)
        .nest("", general_routes)
        .with_state(mint);

    if let Some(ref serve_wallet_path) = server_config.serve_wallet_path {
        return router.nest_service(
            "/",
            get_service(ServeDir::new(serve_wallet_path))
                .layer(middleware::from_fn(add_response_headers)),
        );
    }
    router
}

/// This function adds response headers that are specific to Flutter web applications.
///
/// It sets the `cross-origin-embedder-policy` header to `require-corp` and the
/// `cross-origin-opener-policy` header to `same-origin`. These headers are necessary
/// for some features of Flutter web applications, such as isolating the application
/// from potential security threats in other browsing contexts.
///
/// # Arguments
///
/// * `req` - The incoming request.
/// * `next` - The next middleware or endpoint in the processing chain.
///
/// # Returns
///
/// This function returns a `Result` with the modified response, or an error if
/// something went wrong while processing the request or response.
async fn add_response_headers(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut res = next.run(req).await;

    res.headers_mut().insert(
        HeaderName::from_static("cross-origin-embedder-policy"),
        HeaderValue::from_static("require-corp"),
    );
    res.headers_mut().insert(
        HeaderName::from_static("cross-origin-opener-policy"),
        HeaderValue::from_static("same-origin"),
    );
    Ok(res)
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
            (status = 200, description = "health check")
    ),
)]
async fn get_health() -> impl IntoResponse {
    StatusCode::OK
}

// ######################################################################################################

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        btconchain::MockBtcOnchain,
        config::{DatabaseConfig, MintConfig},
        database::postgres::PostgresDB,
        server::app,
    };
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use moksha_core::{
        keyset::Keysets,
        primitives::{CurrencyUnit, KeysResponse, MintInfoResponse},
    };

    use testcontainers::{clients::Cli, RunnableImage};
    use testcontainers_modules::postgres::Postgres;
    use tower::ServiceExt;

    use crate::{
        config::MintInfoConfig,
        lightning::{LightningType, MockLightning},
        mint::Mint,
    };
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_get_keys() -> anyhow::Result<()> {
        let docker = Cli::default();
        let image = create_postgres_image();
        let node = docker.run(image);

        let app = app(create_mock_mint(Default::default(), node.get_host_port_ipv4(5432)).await?);
        let response = app
            .oneshot(Request::builder().uri("/v1/keys/sat").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await?.to_bytes();
        let keys: KeysResponse = serde_json::from_slice(&body)?;
        assert_eq!(64, keys.keysets[0].keys.len());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_keysets() -> anyhow::Result<()> {
        let docker = Cli::default();
        let image = create_postgres_image();
        let node = docker.run(image);

        let app = app(create_mock_mint(Default::default(), node.get_host_port_ipv4(5432)).await?);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/keysets/sat")
                    .body(Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await?.to_bytes();
        let keysets = serde_json::from_slice::<Keysets>(&body)?;
        assert_eq!(
            Keysets::new("00f545318e4fad2b".to_owned(), CurrencyUnit::Sat, true),
            keysets
        );
        Ok(())
    }

    // FIXME remove duplicated code from mint.rs
    async fn create_mock_db_empty(port: u16) -> anyhow::Result<PostgresDB> {
        let connection_string =
            &format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);
        let db = PostgresDB::new(&DatabaseConfig {
            db_url: connection_string.to_owned(),
            ..Default::default()
        })
        .await?;
        db.migrate().await;
        Ok(db)
    }

    fn create_postgres_image() -> RunnableImage<Postgres> {
        let node = Postgres::default().with_host_auth();
        RunnableImage::from(node).with_tag("16.2-alpine")
    }

    async fn create_mock_mint(info: MintInfoConfig, db_port: u16) -> anyhow::Result<Mint> {
        let db = create_mock_db_empty(db_port).await?;
        let lightning = Arc::new(MockLightning::new());

        Ok(Mint::new(
            lightning,
            LightningType::Lnbits(Default::default()),
            db,
            MintConfig {
                info,
                privatekey: "mytestsecret".to_string(),
                ..Default::default()
            },
            Default::default(),
            Some(Arc::new(MockBtcOnchain::default())),
        ))
    }

    // ################ v1 api tests #####################

    #[tokio::test]
    async fn test_get_keys_v1() -> anyhow::Result<()> {
        let docker = Cli::default();
        let image = create_postgres_image();
        let node = docker.run(image);

        let app = app(create_mock_mint(Default::default(), node.get_host_port_ipv4(5432)).await?);
        let response = app
            .oneshot(Request::builder().uri("/v1/keys/sat").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await?.to_bytes();
        let keys: KeysResponse = serde_json::from_slice(&body)?;
        let keysets = keys.keysets;
        assert_eq!(&1, &keysets.len());
        assert_eq!(64, keysets[0].keys.len());
        assert_eq!(16, keysets[0].id.len());
        assert_eq!(CurrencyUnit::Sat, keysets[0].unit);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_keysets_v1() -> anyhow::Result<()> {
        let docker = Cli::default();
        let image = create_postgres_image();
        let node = docker.run(image);

        let app = app(create_mock_mint(Default::default(), node.get_host_port_ipv4(5432)).await?);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/keysets/sat")
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await?.to_bytes();
        let keysets = serde_json::from_slice::<Keysets>(&body)?;
        assert_eq!(1, keysets.keysets.len());
        assert_eq!(16, keysets.keysets[0].id.len());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_v1_keys() -> anyhow::Result<()> {
        let docker = Cli::default();
        let image = create_postgres_image();
        let node = docker.run(image);

        let app = app(create_mock_mint(Default::default(), node.get_host_port_ipv4(5432)).await?);
        let response = app
            .oneshot(Request::builder().uri("/v1/keys/sat").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await?.to_bytes();
        let keys: KeysResponse = serde_json::from_slice(&body)?;
        assert_eq!(1, keys.keysets.len());
        assert_eq!(
            64,
            keys.keysets.first().expect("keyset not found").keys.len()
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_v1_keys_id_invalid() -> anyhow::Result<()> {
        let docker = Cli::default();
        let image = create_postgres_image();
        let node = docker.run(image);

        let app = app(create_mock_mint(Default::default(), node.get_host_port_ipv4(5432)).await?);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/keys/unknownkeyset/sat/1111")
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_v1_keys_id() -> anyhow::Result<()> {
        let docker = Cli::default();
        let image = create_postgres_image();
        let node = docker.run(image);

        let app = app(create_mock_mint(Default::default(), node.get_host_port_ipv4(5432)).await?);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/keys/00f545318e4fad2b/sat/111")
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await?.to_bytes();
        let keys: KeysResponse = serde_json::from_slice(&body)?;
        assert_eq!(1, keys.keysets.len());
        assert_eq!(
            64,
            keys.keysets.first().expect("keyset not found").keys.len()
        );
        assert_eq!(
            "00f545318e4fad2b",
            keys.keysets.first().expect("keyset not found").id
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_v1_keysets() -> anyhow::Result<()> {
        let docker = Cli::default();
        let image = create_postgres_image();
        let node = docker.run(image);

        let app = app(create_mock_mint(Default::default(), node.get_host_port_ipv4(5432)).await?);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/keysets/sat")
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await?.to_bytes();
        let keys: Keysets = serde_json::from_slice(&body)?;
        assert_eq!(1, keys.keysets.len());
        let keyset = keys.keysets.first().expect("keyset not found");
        assert!(keyset.active);
        assert_eq!(CurrencyUnit::Sat, keyset.unit);
        assert_eq!("00f545318e4fad2b", keyset.id);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_health() -> anyhow::Result<()> {
        let docker = Cli::default();
        let image = create_postgres_image();
        let node = docker.run(image);

        let app = app(create_mock_mint(Default::default(), node.get_host_port_ipv4(5432)).await?);
        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_info() -> anyhow::Result<()> {
        let docker = Cli::default();
        let image = create_postgres_image();
        let node = docker.run(image);

        let mint_info_settings = MintInfoConfig {
            name: Some("Bob's Cashu mint".to_string()),
            version: true,
            description: Some("A mint for testing".to_string()),
            description_long: Some("A mint for testing long".to_string()),
            ..Default::default()
        };
        let app = app(create_mock_mint(mint_info_settings, node.get_host_port_ipv4(5432)).await?);
        let response = app
            .oneshot(Request::builder().uri("/v1/info").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await?.to_bytes();
        let info = serde_json::from_slice::<MintInfoResponse>(&body)?;
        assert_eq!(info.name, Some("Bob's Cashu mint".to_string()));
        assert_eq!(info.description, Some("A mint for testing".to_string()));
        assert_eq!(
            info.description_long,
            Some("A mint for testing long".to_string())
        );
        Ok(())
    }
}
