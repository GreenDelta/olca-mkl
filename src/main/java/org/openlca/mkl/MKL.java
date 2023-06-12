package org.openlca.mkl;

import java.io.File;
import java.util.concurrent.atomic.AtomicBoolean;

public final class MKL {

	private static final AtomicBoolean _loaded = new AtomicBoolean(false);

	public static native int version();

	public static native void denseMatrixVectorMultiplication(
		int rows, int columns, double[] matrix, double[] vector, double[] result
	);

	public static boolean loadFrom(File folder) {
		if (_loaded.get())
			return true;
		if (folder == null)
			return false;
		synchronized (_loaded) {
			if (_loaded.get())
				return true;
			loadLibraries(folder);
			try {
				int v = MKL.version();
				if (v > 0) {
					_loaded.set(true);
					return true;
				}
			} catch (Throwable e) {
				e.printStackTrace(); // TODO: logging!
			}
			return false;
		}
	}

	private static boolean loadLibraries(File folder) {
		var libs = new String[] {
			"mkl_rt.2.dll",
			"olcamkl.dll",
		};
		for (var lib : libs) {
			var file = new File(folder, lib);
			if (!file.exists())
				return false;
			System.load(file.getAbsolutePath());
		}
		return true;
	}
}
