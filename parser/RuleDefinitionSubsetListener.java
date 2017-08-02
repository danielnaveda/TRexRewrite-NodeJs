import org.json.simple.*;
import java.util.HashMap;

public class RuleDefinitionSubsetListener extends RuleDefinitionBaseListener {
    JSONObject obj = new JSONObject();

    HashMap<String, JSONObject> variables = new HashMap<String, JSONObject>();

    // Let's use a counter to keep track of the predicate order
    private int predicate_index = 0;
    private int parameter_index = 0;
    private String event_name = "";

    @Override
    public void enterTesla(RuleDefinitionParser.TeslaContext ctx) {
      obj.put("predicates", new JSONArray());
      obj.put("filters", new JSONArray());
      obj.put("event_template", new JSONObject());
      obj.put("consuming", new JSONArray());
    }

    @Override
    public void exitTesla(RuleDefinitionParser.TeslaContext ctx) {
      System.out.println(obj);
    }

    @Override public void enterFrom(RuleDefinitionParser.FromContext ctx) {
      JSONArray predicates = (JSONArray) obj.get("predicates");

      JSONObject parameters = new JSONObject();
      parameters.put("parameters", new JSONArray());

      JSONObject trigger = new JSONObject();

      trigger.put("Trigger",parameters);

      JSONObject trigger_pred = new JSONObject();
      trigger_pred.put("ty",trigger);

      predicates.add(trigger_pred);
    }

    @Override
    public void enterPredicate_body(RuleDefinitionParser.Predicate_bodyContext ctx) {
      event_name = ctx.getChild(0).getText();

      JSONObject tuple = new JSONObject();

      if (isNumeric(ctx.getChild(0).getText())) {
        tuple.put("ty_id", Integer.parseInt(ctx.getChild(0).getText()));
      } else {
        tuple.put("ty_id", ctx.getChild(0).getText());
      }

      tuple.put("constraints", new JSONArray());
      tuple.put("alias", ctx.alias().getChild(1).getText());

      JSONArray predicates = (JSONArray) obj.get("predicates");
      JSONObject predicate = (JSONObject)predicates.get(predicate_index);

      predicate.put("tuple", tuple);
    }

    @Override
    public void enterEmit(RuleDefinitionParser.EmitContext ctx) {
      predicate_index = -1;

      event_name = ctx.getChild(1).getText();

      JSONObject event_t_obj = (JSONObject) obj.get("event_template");

      if (isNumeric(ctx.getChild(1).getText())) {
        event_t_obj.put("ty_id", Integer.parseInt(ctx.getChild(1).getText()));
      } else {
        event_t_obj.put("ty_id", ctx.getChild(1).getText());
      }

      event_t_obj.put("attributes", new JSONArray());
    }

    @Override
    public void enterPredicate(RuleDefinitionParser.PredicateContext ctx) {
      predicate_index++;
    }

    @Override public void enterEvent(RuleDefinitionParser.EventContext ctx) {
      JSONArray predicates = (JSONArray) obj.get("predicates");

      JSONObject parameters = new JSONObject();
      parameters.put("parameters", new JSONArray());

      JSONObject event = new JSONObject();
      event.put("Event", parameters);

      JSONObject event_pred = new JSONObject();
      event_pred.put("ty",event);

      predicates.add(event_pred);
    }

    @Override public void enterAssignments(RuleDefinitionParser.AssignmentsContext ctx) {
      parameter_index = 0;
    }

    @Override public void enterAssignment(RuleDefinitionParser.AssignmentContext ctx) {
      JSONArray predicates = (JSONArray) obj.get("predicates");
      JSONObject predicate = (JSONObject) predicates.get(predicate_index);
      JSONObject ty = (JSONObject) predicate.get("ty");
      JSONObject Trigger = null;
      if (predicate_index == 0)
        Trigger = (JSONObject) ty.get("Trigger");
      else
        Trigger = (JSONObject) ty.get("Event");

      JSONArray parameters = (JSONArray) Trigger.get("parameters");

      JSONObject attribute = new JSONObject();

      if (isNumeric(ctx.getChild(2).getText())){
        attribute.put("attribute", Integer.parseInt(ctx.getChild(2).getText()));
      } else {
        attribute.put("attribute", event_name + "::" + ctx.getChild(2).getText());
      }

      JSONObject reference = new JSONObject();reference.put("Reference", attribute);

      JSONObject parameter = new JSONObject();
      parameter.put("name", ctx.getChild(0).getText());
      parameter.put("expression",
                    reference
                    );

      parameters.add(parameter);

      JSONObject var_parameter_cont = new JSONObject();
      var_parameter_cont.put("predicate", new Integer(predicate_index));

      var_parameter_cont.put("parameter", new Integer(parameter_index));

      JSONObject var_parameter = new JSONObject();
      var_parameter.put("Parameter", var_parameter_cont);

      variables.put(ctx.getChild(0).getText(), var_parameter);

      parameter_index++;
    }

    @Override public void enterEvent_selection(RuleDefinitionParser.Event_selectionContext ctx) {
      JSONArray predicates = (JSONArray) obj.get("predicates");
      JSONObject predicate = (JSONObject) predicates.get(predicate_index);
      JSONObject ty = (JSONObject) predicate.get("ty");
      JSONObject Trigger = null;
      if (predicate_index == 0)
        Trigger = (JSONObject) ty.get("Trigger");
      else
        Trigger = (JSONObject) ty.get("Event");

      Trigger.put("selection", ctx.getChild(0).getText().substring(0, 1).toUpperCase() + ctx.getChild(0).getText().substring(1));
    }

    @Override public void enterWithin(RuleDefinitionParser.WithinContext ctx) {
      JSONArray predicates = (JSONArray) obj.get("predicates");
      JSONObject predicate = (JSONObject) predicates.get(predicate_index);
      JSONObject ty = (JSONObject) predicate.get("ty");
      JSONObject Trigger = null;
      if (predicate_index == 0)
        Trigger = (JSONObject) ty.get("Trigger");
      else
        Trigger = (JSONObject) ty.get("Event");

      JSONObject bound = new JSONObject();
      bound.put("Within", new JSONObject());
      JSONObject timing = new JSONObject();
      timing.put("upper", new Integer(0));
      timing.put("bound", bound);

      Trigger.put("timing", timing);
    }

    @Override public void enterTime(RuleDefinitionParser.TimeContext ctx) {
      JSONArray predicates = (JSONArray) obj.get("predicates");
      JSONObject predicate = (JSONObject) predicates.get(predicate_index);
      JSONObject ty = (JSONObject) predicate.get("ty");
      JSONObject Trigger = null;
      if (predicate_index == 0)
        Trigger = (JSONObject) ty.get("Trigger");
      else
        Trigger = (JSONObject) ty.get("Event");

      JSONObject timing = (JSONObject) Trigger.get("timing");
      JSONObject bound = (JSONObject) timing.get("bound");
      JSONObject Within = (JSONObject) bound.get("Within");
      Within.put("window",Integer.parseInt(ctx.float_t().getText()));
    }

    @Override public void enterExpression(RuleDefinitionParser.ExpressionContext ctx) {
      if (predicate_index >= 0) {
        JSONArray predicates = (JSONArray) obj.get("predicates");
        JSONObject predicate = (JSONObject) predicates.get(predicate_index);
        JSONObject ty = (JSONObject) predicate.get("tuple");
        JSONArray constraints = (JSONArray) ty.get("constraints");

        JSONObject constraint = new JSONObject();

        JSONObject binary_operation = new JSONObject();

        JSONObject right = new JSONObject();

        if (isNumeric(ctx.getChild(2).getText())){
          JSONObject value = new JSONObject();
          value.put("type", "Int");
          value.put("value", ctx.getChild(2).getText());

          JSONObject immediate = new JSONObject();
          immediate.put("value", value);

          right.put("Immediate", immediate);
        } else {
          JSONObject parameter = variables.get(ctx.getChild(2).getText());
          right = parameter;
        }

        JSONObject reference = new JSONObject();

        if (isNumeric(ctx.getChild(0).getText())) {
          reference.put("attribute", Integer.parseInt(ctx.getChild(0).getText()));
        } else {
          reference.put("attribute", event_name+"::"+ctx.getChild(0).getText());
        }

        JSONObject left = new JSONObject();
        left.put("Reference", reference);

        String op_string = new String();

        if(ctx.getChild(1).getText().equals("+")) {
          op_string = "Plus";
        }
        else if(ctx.getChild(1).getText().equals("-")) {
          op_string = "Minus";
        }
        else if(ctx.getChild(1).getText().equals("*")) {
          op_string = "Times";
        }
        else if(ctx.getChild(1).getText().equals("/")) {
          op_string = "Division";
        }
        else if(ctx.getChild(1).getText().equals("=") || ctx.getChild(1).getText().equals("==")) {
          op_string = "Equal";
        }
        else if(ctx.getChild(1).getText().equals("!=")) {
          op_string = "NotEqual";
        }
        else if(ctx.getChild(1).getText().equals(">")) {
          op_string = "GreaterThan";
        }
        else if(ctx.getChild(1).getText().equals(">=")) {
          op_string = "GreaterEqual";
        }
        else if(ctx.getChild(1).getText().equals("<")) {
          op_string = "LowerThan";
        }
        else if(ctx.getChild(1).getText().equals("<=")) {
          op_string = "LowerEqual";
        }

        binary_operation.put("operator", op_string);
        binary_operation.put("left", left);
        binary_operation.put("right", right);

        constraint.put("BinaryOperation", binary_operation);

        constraints.add(constraint);
      } else {
        JSONObject event_t_obj = (JSONObject) obj.get("event_template");
        JSONArray attributes = (JSONArray) event_t_obj.get("attributes");

        JSONObject parameter = variables.get(ctx.getChild(2).getText());

        attributes.add(parameter);
      }
    }

    public boolean isNumeric(String s) {
        return java.util.regex.Pattern.matches("\\d+", s);
    }
}
