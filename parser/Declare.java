import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.tree.*;
import java.io.FileInputStream;
import java.io.InputStream;

// Build and run
// javac Declare.java Declare*.java
// java Declare "declare SMOKE(value:string,val:int,temperature:int) with id 48"
public class Declare {
  public static void main(String[] args) throws Exception {
    // String inputFile = null;
    // if ( args.length>0 ) inputFile = args[0];
    // InputStream is = System.in;
    // if ( inputFile!=null ) is = new FileInputStream(inputFile);
    // ANTLRInputStream input = new ANTLRInputStream(is);
    // ANTLRInputStream input = new ANTLRInputStream("declare FIRE(value:string,val:int) with id 25");
    ANTLRInputStream input = new ANTLRInputStream(args[0]);

    DeclareLexer lexer = new DeclareLexer(input);
    // DeclareLexer lexer = new DeclareLexer(new String("Declare declare FIRE(value:string,val:int) with id 25"));
    CommonTokenStream tokens = new CommonTokenStream(lexer);
    DeclareParser parser = new DeclareParser(tokens);
    ParseTree tree = parser.tesla(); // parse; start at prog
    // System.out.println(tree.toStringTree(parser)); // print tree as text
  }
}
