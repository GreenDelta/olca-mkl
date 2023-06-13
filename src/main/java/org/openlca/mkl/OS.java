package org.openlca.mkl;

import java.io.File;
import java.util.Locale;
import java.util.function.Consumer;

enum OS {

	Linux(new String[]{
		"libmkl_rt.so",
		"libolcamkl.so"
	}),

	MacOS(new String[] {
		"libmkl_rt.dylib",
		"libolcamkl.dylib"
	}),

	Windows(new String[] {
		"mkl_rt.2.dll",
		"olcamkl.dll",
	});

	private final String[] libraries;

	OS(String[] libraries) {
		this.libraries = libraries;
	}

	static OS detect() {
		var os = System.getProperty("os.name", "generic")
			.toLowerCase(Locale.ENGLISH);
		if (os.contains("mac") || os.contains("darwin"))
			return OS.MacOS;
		if (os.contains("win"))
			return OS.Windows;
		return OS.Linux;
	}

	boolean loadLibrariesFrom(File dir) {
		// TODO: logging
		if (dir == null || !dir.exists()) {
			return false;
		}
		for (var lib : libraries) {
			var libFile = new File(dir, lib);
			if (!libFile.exists()) {
				return false;
			}
			try {
				System.load(libFile.getAbsolutePath());
			} catch (Throwable e) {
				return false;
			}
		}
		return true;
	}
}
