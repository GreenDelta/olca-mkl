package org.openlca.mkl;

import org.openlca.core.matrix.format.CSCMatrix;
import org.openlca.core.matrix.format.HashPointMatrix;
import org.openlca.core.matrix.format.Matrix;
import org.openlca.core.matrix.format.MatrixReader;
import org.openlca.core.matrix.solvers.Factorization;
import org.openlca.core.matrix.solvers.MatrixSolver;

public class MKLSolver implements MatrixSolver {

	@Override
	public boolean hasSparseSupport() {
		return true;
	}

	@Override
	public Matrix matrix(int rows, int columns) {
		return new HashPointMatrix(rows, columns);
	}

	@Override
	public double[] solve(MatrixReader a, int idx, double d) {
		return new double[0];
	}

	@Override
	public Matrix invert(MatrixReader a) {
		return null;
	}

	@Override
	public Matrix multiply(MatrixReader a, MatrixReader b) {
		return MatrixSolver.super.multiply(a, b);
	}

	@Override
	public double[] multiply(MatrixReader m, double[] v) {
		return MatrixSolver.super.multiply(m, v);
	}

	@Override
	public Factorization factorize(MatrixReader matrix) {
		var csc = asSparse(matrix);
		if (csc != null) {
			var ptr = new long[1];
			int error = MKL.sparseFactorization(
				csc.rows, csc.values, csc.rowIndices, csc.columnPointers, ptr);
			// TODO: translate MKL errors to Apache Math
			if (error != 0)
				throw new RuntimeException("MKL error: " + error);
			return new SparseFactorization(ptr[0], csc.rows);
		}

		// TODO: implement for dense matrices
		throw new RuntimeException("not yet implemented");
	}

	@Override
	public boolean isNative() {
		return true;
	}

	private CSCMatrix asSparse(MatrixReader matrix) {
		if (matrix instanceof CSCMatrix csc)
			return csc;
		if (matrix instanceof HashPointMatrix hpm)
			return hpm.compress();
		return null;
	}
}
