grammar Declare;

@header {
  import java.util.HashMap;
}
@members {
  HashMap<String, String> memory = new HashMap<String, String>();
}

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
attribute: attribute_n ':' attribute_t
{memory.put($attribute_n.text, $attribute_t.text);};
attribute_n: LOWER_IDENTIFIER;
attribute_t: 'int' | 'float' | 'bool' | 'string' ;
WITH_ID: 'with' WS 'id';
ID: [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;
