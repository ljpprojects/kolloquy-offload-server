# Kolloquy Offload Server

This is the server Kolloquy uses to securely compute Argon2 hashes.

## Running the Server

The server should be run locally and not visible to the internet directly (use
loopback addresses).

Use nginx as a reverse proxy with SSL.

## Expected Environment Variables & Inputs

- `SECRET_LENGTH` — The lenght of the secret (which is to be read from stdin)
- `ARGON_PEPPER` — The pepper to use during stage 2.

The secret (used during stage 2) is to be piped to the process via stdin at
startup and should be in raw binary form, having the length specified in
`SECRET_LENGTH`.
