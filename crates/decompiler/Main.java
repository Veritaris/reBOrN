import java.util.function.Consumer;

class Main {
    public int publicIntPrimitive;
    public static String publicStaticString;
    public static final String publicStaticFinalString = "Public Static Final String";
    private int privateInt;
    private static String privateStaticString = "Private Static String";
    protected transient String protectedTransientString;
    public static final SomeInnerClass innerClassSingleton = new SomeInnerClass(1, 257);
    public int[][][] array;

    public Main() {
    }

    public Main(int publicIntPrimitiveFormal, String publicStaticStringFormal) {
        this.publicIntPrimitive = publicIntPrimitiveFormal;
        this.publicStaticString = publicStaticStringFormal;
    }

    public static void main(String[] args) {
        System.out.println("Привет, Dreamfinity!馬");
        Main main = new Main();
        main.funcThatAcceptsLambda((a) -> {
        });
    }

    protected SomeInnerClass getInnerClassInstance(int a, int b) {
        return new SomeInnerClass(a, b);
    }

    void funcThatAcceptsLambda(Consumer<SomeInnerClass> consumer) {
        consumer.accept(getInnerClassInstance(1, 2));
    }

    static class SomeInnerClass {
        int fieldA;
        int fieldB;

        public SomeInnerClass(int a, int b) {
            this.fieldA = a;
            this.fieldB = b;
        }

        static class SomeInnerInnerClass {
        }
    }
}
