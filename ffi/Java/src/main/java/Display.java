public class Display extends Quick3DNativeWrapper {
	
	SWIGTYPE_p_void data;	
	Display(int screenWidth, int screenHeight, String title) {
		data = quick3dwrapper.create_display(screenWidth, screenHeight, "My JNI Window");
	}
	
	public void finalize() {
		quick3dwrapper.free_display(data);
	}
}