// Copyright (C) 2016 Chris Liebert

public class Display extends Quick3DNativeWrapper {
	
	Display(int screenWidth, int screenHeight, String title, boolean visible) {
        if(visible) {
			data = quick3dwrapper.create_display(screenWidth, screenHeight, title);
		} else {
			data = quick3dwrapper.create_hidden_display(screenWidth, screenHeight, title);
		}
	}
		
	@Override
	public void dispose() {
		if(null != data) {
			quick3dwrapper.free_display(data);
			data = null;
		}
	}

	public void hide() {
		quick3dwrapper.window_hide(data);
	}

	public void show() {
		quick3dwrapper.window_show(data);		
	}
}