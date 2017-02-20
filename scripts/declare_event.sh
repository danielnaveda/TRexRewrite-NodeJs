#!/bin/bash

# BLUE='\033[0;34m'
# GREEN='\033[0;32m'
# YELLOW='\033[1;33m'
# NC='\033[0m'
#
# printf "${GREEN}--- DECLARE EVENT ---\n"
# printf "${BLUE}Sent: POST /declare-event/${1}\n";
# printf "$(< sample_json/declare-event.json)\n";
# printf "${YELLOW}Received:\n"
# # curl -H "Content-Type: application/json" -X POST -d "$(< sample_json/declare-event.json)" http://127.0.0.1:8888/declare-event/${1}; echo; echo;printf ${NC}
# curl -H "Content-Type: application/json" -X POST -d "$(< sample_json/declare-event.json)" http://127.0.0.1:8888/declare-event; echo; echo;printf ${NC}



#========================================================================================
export CLASSPATH=".:/usr/local/lib/antlr-4.5.3-complete.jar:$CLASSPATH"
alias antlr4='java -Xmx500M -cp "/usr/local/lib/antlr-4.5.3-complete.jar:$CLASSPATH" org.antlr.v4.Tool'
alias grun='java org.antlr.v4.gui.TestRig'

BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

DECLARE_JSON=$(cd ../parser; /usr/bin/java Declare "$1")
#Replace ' for "
# DECLARE_JSON=$(echo $DECLARE_JSON | tr "'" '"')

printf "${GREEN}--- DECLARE EVENT ---\n"
printf "${BLUE}Sent: POST /declare-event\n";
# printf "$(< sample_json/declare-event.json)\n";
printf "$DECLARE_JSON\n";
printf "${YELLOW}Received:\n"
# curl -H "Content-Type: application/json" -X POST -d "$(< sample_json/declare-event.json)" http://127.0.0.1:8888/declare-event/${1}; echo; echo;printf ${NC}
curl -H "Content-Type: application/json" -X POST -d "$DECLARE_JSON" http://127.0.0.1:8888/declare-event; echo; echo;printf ${NC}
# curl -H "Content-Type: application/json" -X POST -d "{\"ty\": \"Event\",\"id\":55,\"name\": \"SMOKE\",\"attributes\": [{\"value\":\"string\",\"temperature\":\"int\",\"val\":\"int\"}]}" http://127.0.0.1:8888/declare-event; echo; echo;printf ${NC}
