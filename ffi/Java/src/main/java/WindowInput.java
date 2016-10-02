// Copyright (C) 2016 Chris Liebert

public class WindowInput extends Quick3DNativeWrapper {
	Input data;
	
	WindowInput(Display display) {
		data = quick3dwrapper.poll_event(display.data);
	}
	
	public void finalize() {
		if(null != data) {
			quick3dwrapper.free_event(data);
			data = null;
		}
	}
}