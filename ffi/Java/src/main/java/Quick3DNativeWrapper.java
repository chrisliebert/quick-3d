// Copyright (C) 2016 Chris Liebert

public abstract class Quick3DNativeWrapper {
	
	protected SWIGTYPE_p_void data = null;
	
	static {
		try {
			// Load libquick3dwrapper.so on Unix and libquick3dwrapper.dll on Windows
			System.loadLibrary("quick3dwrapper");
			System.out.println("Loaded quick3dwrapper shared library");
		} catch (UnsatisfiedLinkError e) {
			System.err.println("Native code library failed to load. " + e);
			System.exit(1);
		}
	}
	
	public abstract void dispose();
	public void finalize() { dispose(); };
	public SWIGTYPE_p_void getPointer() { return data; }
}
