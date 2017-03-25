grammar RuleDefinition;

@header {
  import java.util.HashMap;
  import java.util.Vector;
}
@members {
  /*HashMap<String, String> memory = new HashMap<String, String>();*/

  /*Variables for Predicate Body*/
  HashMap<String, String> predicate_body_parameters = new HashMap<String, String>();
  HashMap<String, String> predicate_body_constraints = new HashMap<String, String>();

  /*Variables for Predicates*/
  Vector<HashMap<String, String>> predicates_parameters = new Vector<HashMap<String, String>>();
  Vector<HashMap<String, String>> predicates_constraints = new Vector<HashMap<String, String>>();

  /*Variable for Emit*/
  HashMap<String, String> emit_parameters = new HashMap<String, String>();

  String predicates = "";
  String event_template = "";
  String filters = "";
  String consuming = "";
}

tesla: event_ids? from where? emit consuming?
{
  /*System.out.println(
    "{"+
        "\"predicates\": "+ predicates +","+
        "\"filters\": "+ filters +","+
        "\"event_template\": "+ event_template +","+
        "\"consuming\": "+ consuming +
    "}"
  );*/
}
;

event_ids: 'EventTypes' definition (',' definition)* ';' ;
definition: CAPITAL_IDENTIFIER '=' float_t;
from: 'from' predicate_body predicates;
//predicates: 'and' predicate predicates | 'and' predicate;
predicates: ('and' predicate)*;
//predicate: event | aggregate | static;
predicate: event;

where: 'where' filters;
filters: expression filters_tail;
filters_tail: 'and' expression filters_tail;

/*emit: 'emit' CAPITAL_IDENTIFIER evaluations*/
/*emit: 'emit' (CAPITAL_IDENTIFIER | IMMEDIATE) evaluations*/
emit: 'emit' (CAPITAL_IDENTIFIER | LOWER_IDENTIFIER | IMMEDIATE) evaluations
{
  /*HashMap<String, String> */
  /*emit_evaluations
  $CAPITAL_IDENTIFIER.text*/

  /*String evaluations_var = "";
  for (String key: emit_evaluations.keySet()){
            String value = emit_evaluations.get(key);

            //input: value
            //output: predicate and parameter

  }

  System.out.println(
    "{"+
        "\"ty_id\": ["+ $CAPITAL_IDENTIFIER.text +"],"+
        "\"attributes\": ["+ attributes +"]"+
    "}"
  );

    String json_parameter =
    "\"Parameter\": {"+
      "\"predicate\":"+ predicate +","+
      "\"parameter\":"+ parameter +","+
    "}";

    String json_parameters =
    "{"+ json_parameters_group +
    "}";*/
}
;
//evaluations: '(' evaluation evaluations_tail ')';
evaluations: '(' evaluation (',' evaluation) ')';
//evaluations_tail: ',' evaluation evaluations_tail;
//evaluation: LOWER_IDENTIFIER '=' expression;
evaluation: expression;

consuming: 'consuming' CAPITAL_IDENTIFIER CAPITAL_IDENTIFIER;
/*predicate_body: CAPITAL_IDENTIFIER assignments constraints alias*/
/*predicate_body: (CAPITAL_IDENTIFIER | IMMEDIATE) assignments constraints alias*/
predicate_body: (CAPITAL_IDENTIFIER | LOWER_IDENTIFIER | IMMEDIATE) assignments constraints alias
{
  /*predicates =
  "{"+
     "\"ty\": {"+
      "\"Trigger\": {"+
        "\"parameters\": ["+
          $assignments.text+ //TODO: create json for this
        "]"+
      "}"+
    "},"+
    "\"tuple\": {"+
      "\"ty_id\":"+ $CAPITAL_IDENTIFIER.text +","+
      "\"constraints\":["+
        //TODO: Fill this optional data
       "]"
      "\"alias\":" + $alias.text +
    "}"
  "}";*/
};
//assignments: '[' assignment assignments_tail ']';
assignments: '[' assignment']';
//assignments_tail: ',' assignment assignments_tail;
//assignment: parameter '=' expression;
assignment: parameter '=' parameter;
//constraints: '(' expression constraints_tail ')' | '(' expression ')' | '(' ')';
constraints: '(' expression (',' expression)* ')' | '(' ')';
constraints_tail: ',' expression constraints_tail;
/*alias: 'as' CAPITAL_IDENTIFIER;*/
alias: 'as' (CAPITAL_IDENTIFIER | LOWER_IDENTIFIER);

event: event_selection predicate_body timing;
event_selection: 'each' | 'not' | 'first' | 'last';

aggregate: aggregate_assignment aggregate_body;
aggregate_assignment: parameter '=';
aggregate_body: aggregator '(' constrained_tuple aggregate_timing')';
aggregator: 'AVG' | 'SUM' | 'MAX' | 'MIN' | 'COUNT';
constrained_tuple: CAPITAL_IDENTIFIER '(' constraints ')' attribute_selection;
aggregate_timing: timing;
attribute_selection: '.' LOWER_IDENTIFIER;

static_t: unordered_static | ordered_static;
unordered_static: unordered_selection predicate_body;
unordered_selection: 'each' | 'not';
ordered_static: ordered_selection predicate_body ordered_by;
ordered_selection: 'first' | 'last';
ordered_by: 'ordered by' ordering orderings;
ordering: LOWER_IDENTIFIER order;
orderings: ',' ordering orderings;
order: 'asc' | 'desc';

timing: within | between;
/*within: 'within' time 'from' CAPITAL_IDENTIFIER;*/
within: 'within' time 'from' (CAPITAL_IDENTIFIER | LOWER_IDENTIFIER);
between: 'between' CAPITAL_IDENTIFIER 'and' CAPITAL_IDENTIFIER;
time: float_t time_unit;
time_unit: 'd' | 'h' | 'min' | 's' | 'ms' | 'us';

//expression: parenthesization | operation | atom;
//expression: operation | atom;
//expression: operation | atom;
expression: parameter operator parameter | parameter operator IMMEDIATE ;
operator: '=' | '==' | '>' | '<' ;
parenthesization: '(' expression ')';
operation: binary_operation | unary_operation;
//binary_operation: expression BINARY_OPERATOR | expression;
binary_operation: BINARY_OPERATOR;
unary_operation: UNARY_OPERATOR expression;
BINARY_OPERATOR: '+' | '*';
UNARY_OPERATOR: '++' | '--' | '==' | '>' | '<';
atom: identifier | parameter | IMMEDIATE;
identifier: qualifier LOWER_IDENTIFIER;
qualifier: CAPITAL_IDENTIFIER '.';

CAPITAL_IDENTIFIER: [A-Z]+ ;
LOWER_IDENTIFIER: [a-z]+ ;
/*parameter: LOWER_IDENTIFIER;*/
parameter: LOWER_IDENTIFIER | IMMEDIATE;
capital_identifiers: ',' capital_identifiers;
lower_identifiers: ',' LOWER_IDENTIFIER lower_identifiers;
IMMEDIATE: [0-9]+ ;
float_t: IMMEDIATE ;
//FLOAT: [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;
