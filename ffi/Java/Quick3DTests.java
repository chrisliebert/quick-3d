public class Quick3DTests {
	Quick3DTests() {}
	
	public void displayCreationTest() {
		Display display = new Display(640, 480, "My JNI Window");
		display.finalize();
	}
	
	public static void main(String[] args) {
		Quick3DTests tests = new Quick3DTests();
		tests.displayCreationTest();
	}
}