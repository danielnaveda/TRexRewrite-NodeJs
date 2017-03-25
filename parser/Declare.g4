grammar Declare;

@header {
  import java.util.HashMap;
}
@members {
  /** Map variable name to Integer object holding value */
  HashMap<String, String> memory = new HashMap<String, String>();
}

// declare FIRE(value:string,val:int) with id 25
/*tesla: expression IDENTIFIER '(' attributes ')' WITH_ID ID*/
tesla: expression identifier '(' attributes ')' WITH_ID ID
{
  String attributes_var = "";
  for (String key: memory.keySet()){
            String value = memory.get(key);
            if (value.equals("int")) {
              value = "Int";
            } else if (value.equals("float")) {
              value = "Float";
            } else if (value.equals("bool")) {
              value = "Bool";
            } else {
              value = "Str";
            }
            if (attributes_var == "")
              attributes_var = "{\"name\":\"" + key + "\"," + "\"ty\":\"" + value + "\"}";
            else
              attributes_var = attributes_var + ",{\"name\":\"" + key + "\"," + "\"ty\":\"" + value + "\"}";


/*'int' => Int
'float' => Float
'bool' => Bool
'string' => Str*/

  }

  System.out.println(
  "{"+
    "\"ty\": \"Event\","+
    "\"id\":"+$ID.text+","+
    "\"name\": \""+$identifier.text+"\","+
    "\"attributes\": ["+attributes_var+"]"+
   "}"
  );
}
;
expression: 'declare' | 'declare fact';
identifier: IDENTIFIER | LOWER_IDENTIFIER;
IDENTIFIER: [A-Z]+ ;
LOWER_IDENTIFIER: [a-z]+ ;
attributes: attribute (',' attribute)*;
/*attribute: ATTRIBUTE_NAME ':' attribute_t {memory.put($ATTRIBUTE_NAME.text, $attribute_t.text);};*/
attribute: attribute_n ':' attribute_t
{memory.put($attribute_n.text, $attribute_t.text);};
attribute_n: LOWER_IDENTIFIER;
/*ATTRIBUTE_NAME: [a-z]+;*/
attribute_t: 'int' | 'float' | 'bool' | 'string' ;
WITH_ID: 'with' WS 'id';
ID: [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;

/*
r: DECLARE IDENTIFIER '(' ATTRIBUTE ')' WITH_ID ID
{
System.out.println(
"{'ty': 'Event','id':"+$ID.text+",'name': '"+$IDENTIFIER.text+"','attributes': [{'name':'string'}]}"
);
}
;
DECLARE: 'declare' | 'declare fact';
IDENTIFIER: [A-Z]+ ;
ATTRIBUTE: ATTRIBUTE_NAME ':' ATTRIBUTE_T {System.out.println($ATTRIBUTE_T.text);};
ATTRIBUTE_NAME: [a-z]+;
ATTRIBUTE_T: 'int' | 'float' | 'bool' | 'string' ;
WITH_ID: 'with' WS 'id';
ID: [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;
*/
