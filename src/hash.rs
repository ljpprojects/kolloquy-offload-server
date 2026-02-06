use std::{env, sync::Arc};

use argon2::{Algorithm, Argon2, AssociatedData, ParamsBuilder, PasswordHash, PasswordHasher, Version};
use base64::{Engine, prelude::BASE64_STANDARD};
use tokio::time::Instant;
use tracing::info;
use zeroize::Zeroize;
use crate::state::ServerState;

pub const HASH_LEN: usize = 48;

/// Stage 2 of password hash algorithm. Computes an argon2id hash with a
/// pepper (30b, environment variable) and secret (384b, received from an
/// external host). Stage 2 receives the digest computed during stage 1 and the
/// salt so that the password is sent plaintext (over HTTPS) only to the
/// Cloudflare worker. HTTPS should still be used when the server is exposed to
/// the internet (via. NGINX reverse proxy).
///
/// Returns the PHC string.
///
/// Stage 1 is the computation of a SHA256 hash (on the Cloudflare Worker) with
/// a per-user salt (32b), and 2 global peppers (32b each, one in .env, one in 
/// Secrets Store).
pub fn stage_2_digest(stage_1_digest: [u8; 32], salt: [u8; 32], state: Arc<ServerState>) -> String {
    let pepper = env::var("ARGON_PEPPER").unwrap();
    let mut pepper = BASE64_STANDARD.decode(pepper).unwrap();

    // The external secret with pepper appended to it
    let mut secret = [&*state.argon_secret, &*pepper].concat();

    let algorithm = Algorithm::Argon2id;
    let version = Version::V0x13;

    let params = ParamsBuilder::new()
        .m_cost(2u32.pow(16)) // 64MiB
        .t_cost(3)
        .p_cost(1)
        .output_len(HASH_LEN)
        .build()
        .unwrap();

    let ctx = Argon2::new_with_secret(&*secret, algorithm, version, params).unwrap();
    let start = Instant::now();

    let phc =
        ctx.hash_password_with_salt(&stage_1_digest, &salt).unwrap().to_string();

    // Does this even do anything, considering every other precaution taken?
    secret.zeroize();
    pepper.zeroize();

    info!(
        time = format!("{:?}", start.elapsed()),
        "Hashing complete",
    );

    phc
}
