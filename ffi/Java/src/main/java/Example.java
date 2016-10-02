// Copyright (C) 2016 Chris Liebert

public class Example {
	
	public static void main(String[] args) {
		int screenWidth = 640, screenHeight = 480;
		Display display = new Display(screenWidth, screenHeight, "My JNI Window");
		boolean done = false;
		WindowInput input;
		while(!done) {
			input = new WindowInput(display);
			if(input.data.getEscape() || input.data.getClosed()) {
				done = true;
			}
		}
		display.finalize();
	}
}
