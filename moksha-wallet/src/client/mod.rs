use async_trait::async_trait;
use moksha_core::{
    blind::BlindedMessage,
    keyset::Keysets,
    primitives::{
        CurrencyUnit, GetMeltBtcOnchainResponse, KeysResponse, MintInfoResponse,
        PostMeltBolt11Response, PostMeltBtcOnchainResponse, PostMeltQuoteBolt11Response,
        PostMeltQuoteBtcOnchainResponse, PostMintBitcreditResponse, PostMintBolt11Response,
        PostMintBtcOnchainResponse, PostMintQuoteBitcreditResponse, PostMintQuoteBolt11Response,
        PostMintQuoteBtcOnchainResponse, PostRequestToMintBitcreditResponse, PostSwapResponse,
    },
    proof::Proofs,
};

use url::Url;

use crate::error::MokshaWalletError;

pub mod crossplatform;

#[cfg(test)]
use mockall::automock;
use moksha_core::primitives::{BillKeys, CheckBitcreditQuoteResponse};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait CashuClient {
    async fn get_keys(
        &self,
        mint_url: &Url,
        unit: String,
    ) -> Result<KeysResponse, MokshaWalletError>;

    async fn get_keys_by_id(
        &self,
        mint_url: &Url,
        keyset_id: String,
        unit: String,
        maturity_date: i64,
    ) -> Result<KeysResponse, MokshaWalletError>;

    async fn get_keysets(&self, mint_url: &Url, unit: String)
        -> Result<Keysets, MokshaWalletError>;

    async fn get_keysets_by_id(
        &self,
        mint_url: &Url,
        unit: String,
        id: String,
        maturity_date: i64,
    ) -> Result<Keysets, MokshaWalletError>;

    async fn post_swap(
        &self,
        mint_url: &Url,
        proofs: Proofs,
        output: Vec<BlindedMessage>,
    ) -> Result<PostSwapResponse, MokshaWalletError>;

    async fn post_melt_bolt11(
        &self,
        mint_url: &Url,
        proofs: Proofs,
        quote: String,
        outputs: Vec<BlindedMessage>,
    ) -> Result<PostMeltBolt11Response, MokshaWalletError>;

    async fn post_melt_quote_bolt11(
        &self,
        mint_url: &Url,
        payment_request: String,
        unit: CurrencyUnit,
    ) -> Result<PostMeltQuoteBolt11Response, MokshaWalletError>;

    async fn get_melt_quote_bolt11(
        &self,
        mint_url: &Url,
        quote: String,
    ) -> Result<PostMeltQuoteBolt11Response, MokshaWalletError>;

    async fn post_mint_bolt11(
        &self,
        mint_url: &Url,
        quote: String,
        blinded_messages: Vec<BlindedMessage>,
    ) -> Result<PostMintBolt11Response, MokshaWalletError>;

    async fn post_mint_bitcredit(
        &self,
        mint_url: &Url,
        quote: String,
        blinded_messages: Vec<BlindedMessage>,
    ) -> Result<PostMintBitcreditResponse, MokshaWalletError>;

    async fn post_mint_quote_bolt11(
        &self,
        mint_url: &Url,
        amount: u64,
        unit: CurrencyUnit,
    ) -> Result<PostMintQuoteBolt11Response, MokshaWalletError>;

    async fn get_mint_quote_bolt11(
        &self,
        mint_url: &Url,
        quote: String,
    ) -> Result<PostMintQuoteBolt11Response, MokshaWalletError>;

    async fn post_mint_quote_bitcredit(
        &self,
        mint_url: &Url,
        bill_id: String,
        node_id: String,
        amount: u64,
        unit: CurrencyUnit,
    ) -> Result<PostMintQuoteBitcreditResponse, MokshaWalletError>;

    async fn get_mint_quote_bitcredit(
        &self,
        mint_url: &Url,
        quote: String,
    ) -> Result<PostMintQuoteBitcreditResponse, MokshaWalletError>;

    async fn post_request_to_mint_bitcredit(
        &self,
        mint_url: &Url,
        bill_id: String,
        bill_keys: BillKeys,
    ) -> Result<PostRequestToMintBitcreditResponse, MokshaWalletError>;

    async fn check_bitcredit_quote(
        &self,
        mint_url: &Url,
        bill_id: String,
        node_id: String,
    ) -> Result<CheckBitcreditQuoteResponse, MokshaWalletError>;

    async fn get_info(&self, mint_url: &Url) -> Result<MintInfoResponse, MokshaWalletError>;

    async fn is_v1_supported(&self, mint_url: &Url) -> Result<bool, MokshaWalletError>;

    async fn post_mint_onchain(
        &self,
        mint_url: &Url,
        quote: String,
        blinded_messages: Vec<BlindedMessage>,
    ) -> Result<PostMintBtcOnchainResponse, MokshaWalletError>;

    async fn post_mint_quote_onchain(
        &self,
        mint_url: &Url,
        amount: u64,
        unit: CurrencyUnit,
    ) -> Result<PostMintQuoteBtcOnchainResponse, MokshaWalletError>;

    async fn get_mint_quote_onchain(
        &self,
        mint_url: &Url,
        quote: String,
    ) -> Result<PostMintQuoteBtcOnchainResponse, MokshaWalletError>;

    async fn post_melt_onchain(
        &self,
        mint_url: &Url,
        proofs: Proofs,
        quote: String,
    ) -> Result<PostMeltBtcOnchainResponse, MokshaWalletError>;

    async fn post_melt_quote_onchain(
        &self,
        mint_url: &Url,
        address: String,
        amount: u64,
        unit: CurrencyUnit,
    ) -> Result<Vec<PostMeltQuoteBtcOnchainResponse>, MokshaWalletError>;

    async fn get_melt_quote_onchain(
        &self,
        mint_url: &Url,
        quote: String,
    ) -> Result<PostMeltQuoteBtcOnchainResponse, MokshaWalletError>;

    async fn get_melt_onchain(
        &self,
        mint_url: &Url,
        txid: String,
    ) -> Result<GetMeltBtcOnchainResponse, MokshaWalletError>;
}
