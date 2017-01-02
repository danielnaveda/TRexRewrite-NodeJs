grammar Declare;
r: DECLARE IDENTIFIER '(' ATTRIBUTE ')' WITH_ID;
DECLARE: 'declare' | 'declare fact' ;
IDENTIFIER: [A-Z]+ ;
ATTRIBUTE: [a-z]+ ':' ATTRIBUTE_TYPE ;
ATTRIBUTE_TYPE: 'int' | 'float' | 'bool' | 'string' ;
WITH_ID: 'with' [ ]+ 'id' [ ]+  ID;
ID: [0-9]+;
WS : [ \t\r\n]+ -> skip ;
