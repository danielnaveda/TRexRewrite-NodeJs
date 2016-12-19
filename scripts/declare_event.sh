#!/bin/bash
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

printf "${GREEN}--- DECLARE EVENT ---\n"
printf "${BLUE}Sent: POST /declare-event/${1}\n";
printf "$(< sample_json/declare-event.json)\n";
printf "${YELLOW}Received:\n"
curl -H "Content-Type: application/json" -X POST -d "$(< sample_json/declare-event.json)" http://127.0.0.1:8888/declare-event/${1}; echo; echo;printf ${NC}
