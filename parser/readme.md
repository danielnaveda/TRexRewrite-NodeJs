. ./set_antlr.sh

antlr4 Declare.g4
javac Declare*.java

java Declare "declare SMOKE(value:string,val:int,temperature:int) with id 55"
{"ty": "Event","id":55,"name": "SMOKE","attributes": [{"value":"string","temperature":"int","val":"int"}]}


antlr4 RuleDefinition.g4
javac RuleDefinition*.java

java RuleDefinition "from SMOKE[x = area]() as SMK and last TEMPERATURE[y = value](area == x, value > 45) as TEMP within 5min from SMK emit FIRE(area = x, temp = y)"
