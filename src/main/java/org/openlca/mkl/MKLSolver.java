package org.openlca.mkl;

import org.openlca.core.matrix.format.CSCMatrix;
import org.openlca.core.matrix.format.DenseMatrix;
import org.openlca.core.matrix.format.HashPointMatrix;
import org.openlca.core.matrix.format.Matrix;
import org.openlca.core.matrix.format.MatrixReader;
import org.openlca.core.matrix.solvers.Factorization;
import org.openlca.core.matrix.solvers.MatrixSolver;

import java.util.Optional;

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
		int n = a.rows();
		var b = new double[n];
		b[idx] = d;

		var csc = asSparse(a).orElse(null);
		if (csc != null) {
			var x = new double[n];
			int info = MKL.solveSparse(
				n,
				csc.values,
				csc.rowIndices,
				csc.columnPointers,
				b,
				x
			);
			InfoCode.checkPardiso(info);
			return x;
		}

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
		var csc = asSparse(matrix).orElse(null);
		var ptr = new long[1];
		if (csc != null) {
			int info = MKL.sparseFactorization(
				csc.rows,
				csc.values,
				csc.rowIndices,
				csc.columnPointers,
				ptr
			);
			InfoCode.checkPardiso(info);
			return new SparseFactorization(ptr[0], csc.rows);
		}

		var dense = DenseMatrix.of(matrix);
		int info = MKL.denseFactorization(dense.rows, dense.data, ptr);
		// TODO: translate MKL errors to Apache Math
		if (info != 0)
			throw new RuntimeException("MKL error: " + info);
		return new DenseFactorization(ptr[0], dense.rows);
	}

	@Override
	public boolean isNative() {
		return true;
	}

	private Optional<CSCMatrix> asSparse(MatrixReader matrix) {
		if (matrix instanceof CSCMatrix csc)
			return Optional.of(csc);
		if (matrix instanceof HashPointMatrix hpm)
			return Optional.of(hpm.compress());
		return Optional.empty();
	}
}
