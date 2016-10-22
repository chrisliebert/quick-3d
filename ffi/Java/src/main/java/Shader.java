import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;

// Copyright (C) 2016 Chris Liebert

public class Shader extends Quick3DNativeWrapper {

	private Display display;
	private String vertexFilename, fragmentFilename;
	private String vertexSource, fragmentSource;
	
	Shader(String vertexFilename, String fragmentFilename, Display display) throws IOException {
		this.display = display;
		this.vertexFilename = vertexFilename;
		this.fragmentFilename = fragmentFilename;
		vertexSource = loadVertexSource();
		fragmentSource = loadFragmentSource();
		data = quick3dwrapper.get_shader_from_source(vertexSource, fragmentSource, display.getPointer());
	}
	
	private String loadFragmentSource() throws IOException {
		byte[] f = Files.readAllBytes(Paths.get(fragmentFilename));
		return new String(f, StandardCharsets.UTF_8);
	}

	private String loadVertexSource() throws IOException {
		byte[] v = Files.readAllBytes(Paths.get(vertexFilename));
		return new String(v, StandardCharsets.UTF_8);
	}

	@Override
	public void dispose() {
		quick3dwrapper.free_shader(data);
	}

	public void reload() throws IOException {
		vertexSource = loadVertexSource();
		fragmentSource = loadFragmentSource();
		if (isValid()) data = quick3dwrapper.get_shader_from_source(vertexSource, fragmentSource, display.getPointer());
	}

	public boolean isValid() {
		return quick3dwrapper.shader_source_is_valid(vertexSource, fragmentSource, display.getPointer());
	}

}
