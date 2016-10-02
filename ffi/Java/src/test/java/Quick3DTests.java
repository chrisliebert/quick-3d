import static org.junit.Assert.*;

import org.junit.Test;

public class Quick3DTests {
	
	@Test
	public void displayCreationTest() {
		Display display = new Display(640, 480, "My JNI Window");
		display.finalize();
	}
	
	@Test
	public void compressedBinaryRendererTest() {
		Display display = new Display(640, 480, "compressed binary renderer test");
		assertNotNull(display);
		SWIGTYPE_p_void renderer = quick3dwrapper.create_renderer_from_compressed_binary("../../test.bin.gz", display.getPointer());
		assertNotNull(renderer);
		quick3dwrapper.free_renderer(renderer);
		display.finalize();
	}
}