// Copyright (C) 2016 Chris Liebert

class Example {
	static public void Main() {
		int screenWidth = 640, screenHeight = 480;
		SWIGTYPE_p_void display = quick3dwrapper.create_display(screenWidth, screenHeight, "My C# Window");
		bool running = true;
		while (running)
		{
			Input input = quick3dwrapper.poll_event(display);
			if (input.closed || input.escape)
			{
				running = false;
			}
			quick3dwrapper.free_event(input);
		}
		quick3dwrapper.free_display(display);
	}
}
