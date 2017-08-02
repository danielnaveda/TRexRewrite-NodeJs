# Build ANTLR grammar file

## Set environment variables
```
. ./set_antlr.sh
```

## Build and compile Declare grammar file
```
antlr4 Declare.g4
javac Declare*.java
```

## Test Declare
```
java Declare "declare SMOKE(value:string,val:int,temperature:int) with id 55"
Expected Result: {"ty": "Event","id":55,"name": "SMOKE","attributes": [{"value":"string","temperature":"int","val":"int"}]}
```

## Build and compile RuleDefinition grammar file
```
antlr4 RuleDefinition.g4
javac RuleDefinition*.java
```

## Test RuleDefinition
```
java RuleDefinition "from SMOKE[x = area]() as SMK and last TEMPERATURE[y = value](area == x, value > 45) as TEMP within 5min from SMK emit FIRE(area = x, temp = y)"
```
