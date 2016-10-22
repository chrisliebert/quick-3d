// Copyright (C) 2016 Chris Liebert

class Example {
	static public void Main() {
		int screenWidth = 640, screenHeight = 480;
		SWIGTYPE_p_void display = quick3dwrapper.create_display(screenWidth, screenHeight, "My C# Window");
		bool running = true;
		while (running)
		{
			SWIGTYPE_p_void events = quick3dwrapper.get_events(display);
			if (quick3dwrapper.display_closed(events) || quick3dwrapper.key_pressed(events, KeyCode.ESCAPE))
			{
				running = false;
			}
			quick3dwrapper.free_events(events);
		}
		quick3dwrapper.free_display(display);
	}
}
