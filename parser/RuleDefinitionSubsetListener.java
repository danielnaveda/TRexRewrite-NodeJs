import org.json.simple.*;

public class RuleDefinitionSubsetListener extends RuleDefinitionBaseListener {
    JSONObject obj = new JSONObject();

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
      tuple.put("ty_id", ctx.CAPITAL_IDENTIFIER().getText());
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
      JSONObject event_t_obj = (JSONObject) obj.get("event_template");
      event_t_obj.put("ty_id", ctx.CAPITAL_IDENTIFIER().getText());
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
      System.out.println(ctx.getChild(0).getText());
      System.out.println(ctx.getChild(2).getText());


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

      System.out.println("Trigger: " + (Trigger==null));
      JSONArray parameters = (JSONArray) Trigger.get("parameters");
      System.out.println("parameters: " + (parameters==null));



      JSONObject attribute = new JSONObject();attribute.put("attribute", new Integer(0));
      JSONObject reference = new JSONObject();reference.put("Reference", attribute);

      JSONObject parameter = new JSONObject();
      parameter.put("name", ctx.getChild(0).getText());
      parameter.put("expression",
                    reference
                    );

      parameters.add(parameter);
    }
}
