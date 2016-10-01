// Copyright (C) 2016 Chris Liebert

public class Example {
	
	public static void main(String[] args) {
		int screenWidth = 640, screenHeight = 480;
		Display display = new Display(screenWidth, screenHeight, "My JNI Window");
		WindowInput input = new WindowInput();
		boolean done = false;
		while(!done) {
			input.poll(display);
			if(input.data.getEscape() || input.data.getClosed()) {
				done = true;
			}
			input.finalize();
		}
		display.finalize();
	}
}
