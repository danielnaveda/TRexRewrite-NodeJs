grammar Declare;

@header {
import java.util.HashMap;
}
@members {
/** Map variable name to Integer object holding value */
HashMap<String, String> memory = new HashMap<String, String>();
}

// declare FIRE(value:string,val:int) with id 25
tesla: expression IDENTIFIER '(' attributes ')' WITH_ID ID
{
  /*System.out.println("IDENTIFIER: " + $IDENTIFIER.text);
  System.out.println("ID: " + $ID.text);
  */
  String attributes_var = "";
  for (String key: memory.keySet()){
            String value = memory.get(key);
            //System.out.println(key + " " + value);
            if (attributes_var != "")
              attributes_var = ',' + attributes_var;
            attributes_var = "\"" + key + "\"" + ":" + "\"" + value + "\"" + attributes_var;
  }

  System.out.println(
  "{"+
    "\"ty\": \"Event\","+
    "\"id\":"+$ID.text+","+
    "\"name\": \""+$IDENTIFIER.text+"\","+
    "\"attributes\": [{"+attributes_var+"}]"+
   "}"
  );
}
;
expression: 'declare' | 'declare fact';
IDENTIFIER: [A-Z]+ ;
attributes: attribute (',' attribute)*;
attribute: ATTRIBUTE_NAME ':' attribute_t {memory.put($ATTRIBUTE_NAME.text, $attribute_t.text);};
ATTRIBUTE_NAME: [a-z]+;
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
