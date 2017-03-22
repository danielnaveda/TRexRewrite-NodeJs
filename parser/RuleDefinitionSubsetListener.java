public class RuleDefinitionSubsetListener extends RuleDefinitionBaseListener {

    // private Stack<Scope> scopes;
    //
    // public VarListener() {
    //     scopes = new Stack<Scope>();
    //     scopes.push(new Scope(null));
    // }

    @Override
    public void enterTesla(RuleDefinitionParser.TeslaContext ctx) {
      System.out.println("enterTesla: walking the tree");
    }

    @Override
    public void exitTesla(RuleDefinitionParser.TeslaContext ctx) {
      System.out.println("exitTesla: walking the tree");
    }

    @Override
    public void enterPredicate_body(RuleDefinitionParser.Predicate_bodyContext ctx) {
      System.out.println("enterPredicate_body: walking the tree" + ctx.CAPITAL_IDENTIFIER().getText());
      System.out.println("enterPredicate_body: walking the tree" + ctx.assignments().getText());
    }

    @Override
    public void exitPredicate_body(RuleDefinitionParser.Predicate_bodyContext ctx) {
      System.out.println("enterPredicate_body: walking the tree" + ctx.CAPITAL_IDENTIFIER().getText());
      System.out.println("enterPredicate_body: walking the tree" + ctx.assignments().getText());
      // CAPITAL_IDENTIFIER
      // assignments
    }
}
