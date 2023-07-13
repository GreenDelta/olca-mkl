package org.openlca.mkl;

import java.io.File;
import java.util.concurrent.atomic.AtomicBoolean;

public final class MKL {

	private static final AtomicBoolean _loaded = new AtomicBoolean(false);

	public static native int version();

	public static native void denseMatrixVectorMultiplication(
		int rows, int columns, double[] matrix, double[] vector, double[] result
	);

	/**
	 * Solves `A*x = b` where A is provided in CSC format.
	 *
	 * @param n The number of rows and columns of `A`.
	 * @param a The non-zero values of `A`.
	 * @param ia The row indices of the non-zero values of A.
	 * @param ja The column pointers of `A`.
	 * @param b The right-hand side vector of size `n`.
	 * @param x The solution vector of size `n`.
	 * @return a possible error code or 0 if no error occurred.
	 */
	public static native int solveSparse(
		int n, double[] a, int[] ia, int[] ja, double[] b, double[] x
	);

	public static native int sparseFactorization(
		int n, double[] a, int[] ia, int[] ja, long[] ptr
	);

	public static native int solveSparseFactorization(
		long ptr, double[] b, double[] x
	);

	public static native void disposeSparseFactorization(long ptr);

	public static native int denseFactorization(
		int n, double[] a, long[] ptr
	);

	public static native int solveDenseFactorization(
		long ptr, int nrhs, double[] b
	);

	public static native void disposeDenseFactorization(long ptr);

	public static boolean loadFrom(File folder) {
		if (_loaded.get())
			return true;
		if (folder == null || !folder.exists())
			return false;
		synchronized (_loaded) {
			if (_loaded.get())
				return true;

			var os = OS.detect();
			if (!os.loadLibrariesFrom(folder))
				return false;
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
}
