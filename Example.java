// Copyright (C) 2016 Chris Liebert

class Example {
	static {
		try {
			// Load libquick3dwrapper.so on Unix and libquick3dwrapper.dll on Windows
			System.loadLibrary("quick3dwrapper");
		} catch (UnsatisfiedLinkError e) {
			System.err.println("Native code library failed to load. " + e);
			System.exit(1);
		}
	}
	
	static class Display {
		SWIGTYPE_p_void data;	
		Display(int screenWidth, int screenHeight, String title) {
			data = quick3dwrapper.create_display(screenWidth, screenHeight, "My JNI Window");
		}
		
		public void finalize() {
			System.err.println("Freeing display");
			quick3dwrapper.free_display(data);
		}
	}

	static class WindowInput {
		Input data;
		
		WindowInput() {
			data = new Input();
		}
		
		void poll(Display display) {
			data = quick3dwrapper.poll_event(display.data);
		}
	}

	public static void main(String[] args) {
		int screenWidth = 640, screenHeight = 480;
		Display display = new Display(screenWidth, screenHeight, "My JNI Window");
		WindowInput input = new WindowInput();
		while(!input.data.getEscape()) {
			input.poll(display);
		}
		display.finalize();
	}
}
