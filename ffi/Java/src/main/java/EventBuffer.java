// Copyright (C) 2016 Chris Liebert

public class EventBuffer extends Quick3DNativeWrapper {
	
	EventBuffer(Display display) {
		data = quick3dwrapper.get_events(display.getPointer());
	}
	
	public void dispose() {
		quick3dwrapper.free_events(data);
	}
	
	public boolean closed() {
		return quick3dwrapper.display_closed(data);
	}

	public boolean key_pressed(KeyCode keycode) {
		return quick3dwrapper.key_pressed(data, keycode);
	}
}