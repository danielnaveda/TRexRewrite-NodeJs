import org.json.simple.*;

public class RuleDefinitionSubsetListener extends RuleDefinitionBaseListener {
    JSONObject obj = new JSONObject();

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

    @Override
    public void enterPredicate_body(RuleDefinitionParser.Predicate_bodyContext ctx) {
      // obj.remove("predicates");
      JSONArray list = (JSONArray) obj.get("predicates");
      list.add("New entry");
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
}
