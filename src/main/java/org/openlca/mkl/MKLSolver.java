package org.openlca.mkl;

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
		return null;
	}

	@Override
	public boolean isNative() {
		return true;
	}
}
