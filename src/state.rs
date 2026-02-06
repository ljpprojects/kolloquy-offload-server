use std::sync::Arc;

#[derive(Clone)]
pub struct ServerState {
    /// This is provided to the process at start via stdin.
    /// It should be sourced from an external host, ideally only accessible with
    /// public-key auth and a passphrase. If the server is self-hosted the
    /// external host should be accessible from the LAN of the server only.
    pub argon_secret: Arc<[u8]>,
}
