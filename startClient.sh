#!/bin/bash
#
# Script to run the Rustic Client

# Check if the required number of arguments have been entered
if [[ $# -ne 2 ]]; then
  echo "Error: Invalid argument. $# argument(s) entered; 2 arguments required"
  echo "Usage: Specify the hostname and port number: ./startServer [hostname] [port]"
  exit 1
fi

# Check if port number is in the correct range
if [[ $2 -lt 1 ]] || [[ $2 -gt 65535 ]]; then
  echo "Error: Invalid port number. $2 is not a valid port number"
  echo "Usage: The correct range of ports is 1 to 65535"
  exit 1
fi

# Allow only characters from 0-9
if ! [[ $2 =~ ^-?[0-9]+$ ]]; then
  echo "Error: Invalid port number. Argument $2 is invalid; Only characters from 0-9 permitted"
  echo "Usage: Specify the port number: ./startClient [hostname] 80"
  exit 1
fi

# Check if Cargo is installed
if [[ ! -x $(which cargo) ]]; then
  echo "Error: Cargo not found."
  echo "Usage: Cargo is required to run this program. URL: https://www.rust-lang.org/tools/install"
  exit 1
fi

# Run program
cargo run --bin client $1 $2;
