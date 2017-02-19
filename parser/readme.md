. ./set_antlr.sh

antlr4 Declare.g4
javac Declare*.java

java Declare "declare SMOKE(value:string,val:int,temperature:int) with id 55"{"ty": "Event","id":55,"name": "SMOKE","attributes": [{"value":"string","temperature":"int","val":"int"}]}
