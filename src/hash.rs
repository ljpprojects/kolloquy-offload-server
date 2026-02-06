use std::{env, sync::Arc};

use argon2::{Algorithm, Argon2, AssociatedData, ParamsBuilder, Version};
use base64::{Engine, prelude::BASE64_STANDARD};
use tokio::time::Instant;
use tracing::info;

use crate::state::ServerState;

pub const HASH_LEN: usize = 48;

/// Stage 2 of password hash algorithm. Computes an argon2id hash with a pepper
/// (environment variable) and secret (received from an external host). Stage 2
/// receives the digest computed during stage 1 so that the password is sent
/// plaintext (over HTTPS) only to the Cloudflare worker. HTTPS should still be
/// used when the server is exposed to the internet (NGINX reverse proxy).
///
/// Stage 1 is the computation of a SHA256 hash (on the Cloudflare Worker) with
/// a per-user salt and a global pepper.
pub fn stage_2_digest(stage_1_digest: [u8; 32], state: Arc<ServerState>) -> [u8; HASH_LEN] {
    let pepper = env::var("ARGON_PEPPER").unwrap();
    let pepper = BASE64_STANDARD.decode(pepper).unwrap();

    let secret = &*state.argon_secret;

    let algorithm = Algorithm::Argon2id;
    let version = Version::V0x13;

    let params = ParamsBuilder::new()
        .m_cost(2u32.pow(16)) // 64MiB
        .t_cost(3)
        .p_cost(1)
        .output_len(HASH_LEN)
        .build()
        .unwrap();

    let ctx = Argon2::new_with_secret(secret, algorithm, version, params).unwrap();
    let mut out = [0u8; HASH_LEN];

    let start = Instant::now();

    ctx.hash_password_into(&stage_1_digest, &*pepper, &mut out).unwrap();

    info!(
        time = format!("{:?}", start.elapsed()),
        "Hashing complete",
    );

    out
}
