grammar RuleDefinition;

@header {
  import java.util.HashMap;
}
@members {
  /** Map variable name to Integer object holding value */
  HashMap<String, String> memory = new HashMap<String, String>();
}

////////////// DEFINE RULE
tesla: event_ids from where? emit consuming?
{
  System.out.println(
  "{\"predicates\":[ { \"ty\": { \"Trigger\": { \"parameters\": [ { \"name\": \"x\",\"expression\": { \"Reference\": { \"attribute\": 0 } } } ] } },\"tuple\": { \"ty_id\": 0,\"constraints\": [],\"alias\": \"smk\" } },{ \"ty\": { \"Event\": { \"selection\": \"Last\",\"parameters\": [ { \"name\": \"y\",\"expression\": { \"Reference\": { \"attribute\": 1 } } } ],\"timing\": { \"upper\": 0,\"bound\": { \"Within\": { \"window\": 5 } } } } },\"tuple\": { \"ty_id\": 1,\"constraints\": [ { \"BinaryOperation\": { \"operator\": \"Equal\",\"left\": { \"Reference\": { \"attribute\": 0 } },\"right\": { \"Parameter\": { \"predicate\": 0,\"parameter\": 0 } } } },{ \"BinaryOperation\": { \"operator\": \"GreaterThan\",\"left\": { \"Reference\": { \"attribute\": 1 } },\"right\": { \"Immediate\": { \"value\": { \"type\": \"Int\",\"value\": \"45\" } } } } } ],\"alias\": \"temp\" } } ],\"filters\": [],\"event_template\": { \"ty_id\": 2,\"attributes\": [ { \"Parameter\": { \"predicate\": 0,\"parameter\": 0 } },{ \"Parameter\": { \"predicate\": 1,\"parameter\": 0 } } ] },\"consuming\": []}"
  );
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

emit: 'emit' CAPITAL_IDENTIFIER evaluations;
//evaluations: '(' evaluation evaluations_tail ')';
evaluations: '(' evaluation (',' evaluation) ')';
//evaluations_tail: ',' evaluation evaluations_tail;
//evaluation: LOWER_IDENTIFIER '=' expression;
evaluation: expression;

consuming: 'consuming' CAPITAL_IDENTIFIER CAPITAL_IDENTIFIER;
predicate_body: CAPITAL_IDENTIFIER assignments constraints alias;
//assignments: '[' assignment assignments_tail ']';
assignments: '[' assignment']';
//assignments_tail: ',' assignment assignments_tail;
//assignment: parameter '=' expression;
assignment: parameter '=' parameter;
//constraints: '(' expression constraints_tail ')' | '(' expression ')' | '(' ')';
constraints: '(' expression (',' expression)* ')' | '(' ')';
constraints_tail: ',' expression constraints_tail;
alias: 'as' CAPITAL_IDENTIFIER;

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
within: 'within' time 'from' CAPITAL_IDENTIFIER;
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
parameter: LOWER_IDENTIFIER;
capital_identifiers: ',' capital_identifiers;
lower_identifiers: ',' LOWER_IDENTIFIER lower_identifiers;
IMMEDIATE: [0-9]+ ;
float_t: IMMEDIATE ;
//FLOAT: [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;
