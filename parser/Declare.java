import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.tree.*;
import java.io.FileInputStream;
import java.io.InputStream;

public class Declare {
  public static void main(String[] args) throws Exception {
    ANTLRInputStream input = new ANTLRInputStream(args[0]);

    DeclareLexer lexer = new DeclareLexer(input);
    CommonTokenStream tokens = new CommonTokenStream(lexer);
    DeclareParser parser = new DeclareParser(tokens);
    ParseTree tree = parser.tesla();
  }
}
