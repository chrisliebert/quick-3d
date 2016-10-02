// Copyright (C) 2016 Chris Liebert

public class Display extends Quick3DNativeWrapper {
	private SWIGTYPE_p_void data;
	
	Display(int screenWidth, int screenHeight, String title) {
		data = quick3dwrapper.create_display(screenWidth, screenHeight, "My JNI Window");
	}
	
	public SWIGTYPE_p_void getPointer() { return data; }
	
	public void finalize() {
		if(null != data) {
			quick3dwrapper.free_display(data);
			data = null;
		}
	}
}