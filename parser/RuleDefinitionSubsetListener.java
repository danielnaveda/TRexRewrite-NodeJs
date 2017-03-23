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
    public void exitPredicate_body(RuleDefinitionParser.Predicate_bodyContext ctx) {}
}
