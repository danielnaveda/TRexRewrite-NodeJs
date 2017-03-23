#!/bin/bash

# export CLASSPATH=".:/usr/local/lib/antlr-4.5.3-complete.jar:$CLASSPATH"
export CLASSPATH=".:/usr/local/lib/antlr-4.5.3-complete.jar:/usr/local/lib/json-simple-1.1.jar:$CLASSPATH"

alias antlr4='java -Xmx500M -cp "/usr/local/lib/antlr-4.5.3-complete.jar:$CLASSPATH" org.antlr.v4.Tool'

alias grun='java org.antlr.v4.gui.TestRig'
