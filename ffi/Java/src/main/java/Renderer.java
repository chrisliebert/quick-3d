// Copyright (C) 2016 Chris Liebert

public class Renderer extends Quick3DNativeWrapper {
	private Display display;
	
	Renderer(String filename, Display display) {
		this.display = display;
		this.data = quick3dwrapper.create_renderer_from_compressed_binary(filename, display.getPointer());
	}
	
	@Override
	public void dispose() {
		if(null != data) {
			quick3dwrapper.free_renderer(data);
			data = null;
		}
	}
	
	public void render(Shader shader, Camera camera) {
		quick3dwrapper.render(data,  shader.getPointer(), camera.getPointer(), display.getPointer());
	}
}