package org.openlca.mkl;

import java.io.File;
import java.util.concurrent.atomic.AtomicBoolean;

public final class MKL {

	private static final AtomicBoolean _loaded = new AtomicBoolean(false);

	public static native int version();

	/**
	 * Calculates {@code y := A * x}.
	 *
	 * @param m the number of rows of matrix A.
	 * @param n the number of columns of matrix A.
	 * @param a the matrix A in column-major order.
	 * @param x the vector x of size n.
	 * @param y the vector y of size m.
	 */
	public static native void denseMatrixVectorMul(
		int m, int n, double[] a, double[] x, double[] y
	);

	/**
	 * Calculates {@code C := A * B}.
	 *
	 * @param m the number of rows of matrix A.
	 * @param n the number of columns of matrix B.
	 * @param k the number of rows (columns) of matrix A (B).
	 * @param a the matrix A.
	 * @param b the matrix B.
	 * @param c the matrix C.
	 */
	public static native void denseMatrixMul(
		int m, int n, int k, double[] a, double[] b, double[] c
	);

	/**
	 * Solves x in {@code A * x = b} where A is provided in CSC format.
	 *
	 * @param n the number of rows and columns of A.
	 * @param a the non-zero values of A.
	 * @param ia the row indices of the non-zero values of A.
	 * @param ja the column pointers of A.
	 * @param b the right-hand side vector of size n.
	 * @param x the solution vector of size n.
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

	/**
	 * Solves x in {@code A * x = b}. Note that this method mutates
	 * the parameter A: on exit it will contain the LU-factorization
	 * of the matrix A.
	 *
	 * @param n the number of rows and columns of A.
	 * @param nrhs the number of columns of x and b.
	 * @param a on entry, the matrix A, on exit the factorization of A.
	 * @param b on entry, the right-hand side, on exit the solution x.
	 */
	public static native int solveDense(
		int n, int nrhs, double[] a, double[] b
	);

	/**
	 * Inverts a matrix A in place.
	 *
	 * @param n the number of rows and columns of A.
	 * @param a on entry the matrix A, on exit the inverse of A.
	 * @return 0 on success or an error code otherwise.
	 */
	public static native int invertDense(int n, double[] a);

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
