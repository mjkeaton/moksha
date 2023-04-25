use std::collections::HashMap;

use axum::extract::{Query, State};
use axum::Router;
use axum::{routing::get, Json};
use bitcoin_hashes::{sha256, Hash};
use cashurs_core::dhke;
use cashurs_core::model::{
    BlindedMessage, BlindedSignature, Keysets, MintKeyset, PaymentRequest, PostMintResponse,
};
use hyper::Method;
use model::MintQuery;
use secp256k1::PublicKey;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{event, Level};

use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod model;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    event!(Level::INFO, "startup");

    let addr = "[::]:3338".parse()?;
    event!(Level::INFO, "listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(
            app()
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET, Method::POST]),
                )
                .into_make_service(),
        )
        .await?;

    Ok(())
}

fn app() -> Router {
    let keyset = MintKeyset::new("supersecretprivatekey".to_string());
    Router::new()
        .route("/keys", get(get_keys))
        .route("/keysets", get(get_keysets))
        .route("/mint", get(get_mint).post(post_mint))
        .with_state(keyset)
        .layer(TraceLayer::new_for_http())
}

async fn get_mint(Query(mint_query): Query<MintQuery>) -> Result<Json<PaymentRequest>, ()> {
    println!("amount: {:#?}", mint_query); // FIXME use amount and generate a real invoice
    let pr = "lnbc2500u1pvjluezpp5qqqsyqcyq5rqwzqfqqqsyqcyq5rqwzqfqqqsyqcyq5rqwzqfqypqdq5xysxxatsyp3k7enxv4jsxqzpuaztrnwngzn3kdzw5hydlzf03qdgm2hdq27cqv3agm2awhz5se903vruatfhq77w3ls4evs3ch9zw97j25emudupq63nyw24cg27h2rspfj9srp";
    Ok(Json(PaymentRequest {
        pr: pr.to_string(),
        hash: sha256::Hash::hash(pr.as_bytes()).to_string(),
    }))
}

async fn post_mint(
    State(keyset): State<MintKeyset>,
    Query(mint_query): Query<MintQuery>,
    Json(blinded_messages): Json<Vec<BlindedMessage>>,
) -> Result<Json<PostMintResponse>, ()> {
    println!("post_mint: {mint_query:#?} {blinded_messages:#?}");

    let private_key = keyset.private_keys.get(&2).unwrap();
    let blinded_sig = dhke::step2_bob(blinded_messages[0].b_, private_key).unwrap(); // FIXME

    // FIXME return correct values for keyset and amount
    let result = BlindedSignature {
        id: Some("keyset.".to_string()),
        amount: 2,
        c_: blinded_sig,
    };

    Ok(Json(PostMintResponse {
        promises: vec![result],
    }))
}

async fn get_keys(State(keyset): State<MintKeyset>) -> Result<Json<HashMap<u64, PublicKey>>, ()> {
    Ok(Json(keyset.public_keys))
}

async fn get_keysets(State(keyset): State<MintKeyset>) -> Result<Json<Keysets>, ()> {
    Ok(Json(Keysets {
        keysets: vec![keyset.keyset_id],
    }))
}
