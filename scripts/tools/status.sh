#!/bin/bash
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

#Subscribe
printf "${GREEN}--- STATUS ---\n"
printf "${BLUE}Sent: GET status\n";
printf "{}\n";
printf "${YELLOW}Received:\n"
curl -H "Content-Type: application/json" http://127.0.0.1:8888/status; echo; echo
