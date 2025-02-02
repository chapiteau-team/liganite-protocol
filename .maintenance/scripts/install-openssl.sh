#!/usr/bin/env bash

if command -v openssl &> /dev/null; then
  echo "OpenSSL already installed"
  exit 0
fi

# Determine the operating system
if [[ "$OSTYPE" == "darwin"* ]]; then
  # macOS
  if command -v brew &> /dev/null; then
    echo "Installing OpenSSL using Homebrew..."
    brew install openssl@3
  else
    echo "Homebrew not found. Please install Homebrew first: https://brew.sh/"
    exit 1
  fi
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
  # Linux
  if command -v apt-get &> /dev/null; then
    echo "Installing OpenSSL using apt-get..."
    sudo apt-get update -y
    sudo apt-get install -y gcc pkg-config libssl-dev
  else
    echo "apt-get not found. Falling back to manual installation..."
    manual_install
  fi
else
  # Other Unix-like systems
  echo "Unsupported operating system. Falling back to manual installation..."
  manual_install
fi

manual_install() {
  echo "Manual installation of OpenSSL is not implemented."
  echo "Please install OpenSSL manually for your system."
  exit 1
}

# Verify installation
if command -v openssl &> /dev/null; then
  echo "OpenSSL successfully installed"
  openssl version
else
  echo "Failed to install OpenSSL"
  exit 1
fi
