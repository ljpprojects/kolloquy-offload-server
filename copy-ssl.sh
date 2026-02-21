#! /bin/bash

# This script copies over the certificates for TLS and mTLS (in ./ssl) used in the nginx reverse proxy to /etc/kolloquy
# (where they are expected to be in the nginx config)

sudo mkdir -p /etc/kolloquy

# Copy files

sudo cp ssl/ssl.crt /etc/kolloquy
sudo cp ssl/ssl.pem /etc/kolloquy
sudo cp ssl/ca.crt /etc/kolloquy

# Make sure permissions & ownership are correct

sudo chown root /etc/kolloquy
sudo chown root -R /etc/kolloquy

sudo chmod 0600 -R /etc/kolloquy
sudo chmod 0600 /etc/kolloquy
