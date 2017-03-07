import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.tree.*;
import java.io.FileInputStream;
import java.io.InputStream;

// Build and run
// javac Declare.java Declare*.java
// java Declare "declare SMOKE(value:string,val:int,temperature:int) with id 48"
public class RuleDefinition {
  public static void main(String[] args) throws Exception {
    ANTLRInputStream input = new ANTLRInputStream(args[0]);

    RuleDefinitionLexer lexer = new RuleDefinitionLexer(input);
    CommonTokenStream tokens = new CommonTokenStream(lexer);
    RuleDefinitionParser parser = new RuleDefinitionParser(tokens);
    ParseTree tree = parser.tesla(); // parse; start at prog
  }
}
