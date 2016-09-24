public abstract class Quick3DNativeWrapper {
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
}
