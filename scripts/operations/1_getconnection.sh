#!/bin/bash
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

printf "${GREEN}--- GET CONNECTION ---\n"
printf "${BLUE}Sent: GET /connections\n";
printf "{}\n";
printf "${YELLOW}Received:\n"
OUTPUT=$(curl --fail --silent --show-error http://127.0.0.1:8888/connections)
CONN_ID=$(echo $OUTPUT | jq -r '.value')
echo "$OUTPUT"; echo
