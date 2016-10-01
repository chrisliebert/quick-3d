public class WindowInput extends Quick3DNativeWrapper {
	Input data;
	
	WindowInput() {
		data = new Input();
	}
	
	void poll(Display display) {
		data = quick3dwrapper.poll_event(display.data);
	}
	
	public void finalize() {
		quick3dwrapper.free_event(data);
	}
}