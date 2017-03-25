#!/bin/bash
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

export CLASSPATH=".:/usr/local/lib/antlr-4.5.3-complete.jar:/usr/local/lib/json-simple-1.1.jar:$CLASSPATH"
alias antlr4='java -Xmx500M -cp "/usr/local/lib/antlr-4.5.3-complete.jar:$CLASSPATH" org.antlr.v4.Tool'
alias grun='java org.antlr.v4.gui.TestRig'

DEFINE_RULE_JSON=$(cd ../../parser; /usr/bin/java RuleDefinition "$1")

printf "${GREEN}--- DEFINE RULE ---\n"
printf "${BLUE}Sent: POST /define-rule\n";
printf "$DEFINE_RULE_JSON\n";
printf "${YELLOW}Received:\n"
curl -H "Content-Type: application/json" -X POST -d "$DEFINE_RULE_JSON" http://127.0.0.1:8888/define-rule; echo; echo;printf ${NC}
