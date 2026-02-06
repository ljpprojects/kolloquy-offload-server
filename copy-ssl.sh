#! /bin/bash

# This script copies over the certificates for TLS and mTLS (in ./ssl) used in the nginx reverse proxy to /etc/kolloquy (where they are expected to be)

sudo mkdir -p /etc/kolloquy

# Copy files

sudo cp ssl/ssl.cert /etc/kolloquy
sudo cp ssl/ssl.pem /etc/kolloquy
sudo cp ssl/ca.cert /etc/kolloquy

# Make sure permissions & ownership are correct

sudo chown root /etc/kolloquy
sudo chown root -R /etc/kolloquy

sudo chmod 0600 -R /etc/kolloquy
sudo chmod 0600 /etc/kolloquy
