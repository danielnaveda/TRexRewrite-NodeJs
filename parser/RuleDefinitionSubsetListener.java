import org.json.simple.*;
import java.util.HashMap;

public class RuleDefinitionSubsetListener extends RuleDefinitionBaseListener {
    JSONObject obj = new JSONObject();

    HashMap<String, JSONObject> variables = new HashMap<String, JSONObject>();

    // Let's use a counter to keep track of the predicate order
    private int predicate_index = 0;

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
      // trigger.put("Trigger",new JSONObject());
      trigger.put("Trigger",parameters);

      JSONObject trigger_pred = new JSONObject();
      trigger_pred.put("ty",trigger);

      // predicates.add(new JSONObject());
      predicates.add(trigger_pred);
    }

    @Override
    public void enterPredicate_body(RuleDefinitionParser.Predicate_bodyContext ctx) {
      // obj.remove("predicates");
      // JSONArray list = (JSONArray) obj.get("predicates");
      // list.add("New entry");
      //
      // ctx.CAPITAL_IDENTIFIER().getText()


      JSONObject tuple = new JSONObject();
      // tuple.put("ty_id", ctx.CAPITAL_IDENTIFIER().getText());

      // tuple.put("ty_id", ctx.getChild(0).getText());

      if (isNumeric(ctx.getChild(0).getText())) {
        tuple.put("ty_id", Integer.parseInt(ctx.getChild(0).getText()));
      } else {
        tuple.put("ty_id", ctx.getChild(0).getText());
      }

      tuple.put("constraints", new JSONArray());
      tuple.put("alias", ctx.alias().CAPITAL_IDENTIFIER().getText());

      JSONArray predicates = (JSONArray) obj.get("predicates");
      // JSONObject predicate = (JSONObject)predicates.getJSONObject(predicate_index);
      JSONObject predicate = (JSONObject)predicates.get(predicate_index);
      // predicate.put("ty", new JSONObject());
      predicate.put("tuple", tuple);
    }

    @Override
    public void enterEmit(RuleDefinitionParser.EmitContext ctx) {
      predicate_index = -1;
      JSONObject event_t_obj = (JSONObject) obj.get("event_template");
      // event_t_obj.put("ty_id", ctx.CAPITAL_IDENTIFIER().getText());
      if (isNumeric(ctx.getChild(1).getText())) {
        event_t_obj.put("ty_id", Integer.parseInt(ctx.getChild(1).getText()));
      } else {
        event_t_obj.put("ty_id", ctx.getChild(1).getText());
      }


      event_t_obj.put("attributes", new JSONArray());
    }

    @Override
    public void enterEvaluation(RuleDefinitionParser.EvaluationContext ctx) {
      JSONObject event_t_obj = (JSONObject) obj.get("event_template");
      JSONArray attributes = (JSONArray) event_t_obj.get("attributes");

      JSONObject parameter = new JSONObject();
      JSONObject parameter_content = new JSONObject();

      parameter_content.put("predicate", new Integer(0));
      parameter_content.put("parameter", new Integer(0));

      parameter.put("Parameter", parameter_content);

      attributes.add(parameter);
    }

    @Override
    public void exitPredicate_body(RuleDefinitionParser.Predicate_bodyContext ctx) {}


    @Override
    public void enterPredicate(RuleDefinitionParser.PredicateContext ctx) {
      predicate_index++;
      // JSONArray predicates = (JSONArray) obj.get("predicates");
      // predicates.add(new JSONObject());
    }

    @Override public void enterEvent(RuleDefinitionParser.EventContext ctx) {
      JSONArray predicates = (JSONArray) obj.get("predicates");

      JSONObject parameters = new JSONObject();
      parameters.put("parameters", new JSONArray());

      JSONObject event = new JSONObject();
      // event.put("Event",new JSONObject());
      event.put("Event", parameters);

      JSONObject event_pred = new JSONObject();
      event_pred.put("ty",event);

      // predicates.add(new JSONObject());
      predicates.add(event_pred);
    }

    @Override public void enterAssignment(RuleDefinitionParser.AssignmentContext ctx) {
      // System.out.println(ctx.getChild(0).getText());
      // System.out.println(ctx.getChild(2).getText());


      // JSONObject tuple = new JSONObject();
      // tuple.put("ty_id", ctx.CAPITAL_IDENTIFIER().getText());
      // tuple.put("constraints", new JSONArray());
      // tuple.put("alias", ctx.alias().CAPITAL_IDENTIFIER().getText());


      JSONArray predicates = (JSONArray) obj.get("predicates");
      JSONObject predicate = (JSONObject) predicates.get(predicate_index);
      // System.out.println("predicate_index: " + predicate_index);
      JSONObject ty = (JSONObject) predicate.get("ty");
      JSONObject Trigger = null;
      if (predicate_index == 0)
        Trigger = (JSONObject) ty.get("Trigger");
      else
        Trigger = (JSONObject) ty.get("Event");

      // System.out.println("Trigger: " + (Trigger==null));
      JSONArray parameters = (JSONArray) Trigger.get("parameters");
      // System.out.println("parameters: " + (parameters==null));

      JSONObject attribute = new JSONObject();

      if (isNumeric(ctx.getChild(2).getText())){
        attribute.put("attribute", Integer.parseInt(ctx.getChild(2).getText()));
      } else {
        attribute.put("attribute", ctx.getChild(2).getText());
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
      if(isNumeric(ctx.getChild(2).getText())) {
        var_parameter_cont.put("parameter", Integer.parseInt(ctx.getChild(2).getText()));
      } else {
        var_parameter_cont.put("parameter", ctx.getChild(2).getText());
      }

      JSONObject var_parameter = new JSONObject();
      var_parameter.put("Parameter", var_parameter_cont);

      variables.put(ctx.getChild(0).getText(), var_parameter);
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

      // Trigger.put("selection", ctx.getChild(0).getText());
      Trigger.put("selection", ctx.getChild(0).getText().substring(0, 1).toUpperCase() + ctx.getChild(0).getText().substring(1));

    }

    @Override public void enterWithin(RuleDefinitionParser.WithinContext ctx) {
      // predicates->predicate[predicate_index]->ty->Trigger/Event->timing
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
      // predicates->predicate[predicate_index]->ty->Trigger/Event->timing->bound->Within
      // Add window in minutes
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

      // JSONObject bound = new JSONObject();
      // bound.put("Within", new JSONObject());
      // JSONObject timing = new JSONObject();
      // timing.put("upper", new Integer(0));
      // timing.put("bound", bound);
      //
      //
      // Trigger.put("timing", timing);
    }

    @Override public void enterExpression(RuleDefinitionParser.ExpressionContext ctx) {
      // predicates->predicate[predicate_index]->tuple->constraints
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
          // JSONObject parameter = new JSONObject();

          JSONObject parameter = variables.get(ctx.getChild(2).getText());


          // parameter.put("predicate", new Integer(0));
          // parameter.put("parameter", new Integer(0));

          // right.put("Parameter", parameter);
          right = parameter;
        }

        JSONObject reference = new JSONObject();

        if (isNumeric(ctx.getChild(0).getText())) {
          reference.put("attribute", Integer.parseInt(ctx.getChild(0).getText()));
        } else {
          reference.put("attribute", ctx.getChild(0).getText());
        }

        JSONObject left = new JSONObject();
        left.put("Reference", reference);

        // binary_operation.put("operator", ctx.getChild(1).getText());

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

      }
    }


    public boolean isNumeric(String s) {
        return java.util.regex.Pattern.matches("\\d+", s);
    }
}
