package org.openlca.mkl;

import org.openlca.core.matrix.format.HashPointMatrix;

import java.io.File;

public class Main {

	public static void main(String[] args) {
		var loaded = MKL.loadFrom(new File("bin"));
		if (!loaded) {
			System.out.println("failed to load kernel");
			return;
		}
		System.out.println(MKL.version());
		var y = new double[]{0, 0, 0};
		MKL.denseMatrixVectorMul(
			3,
			3,
			new double[]{1, 2, 3, 4, 5, 6, 7, 8, 9},
			new double[]{1, 2, 3},
			y
		);
		for (double yi : y) {
			System.out.println(yi);
		}

		double[] x = new double[2];
		var m = HashPointMatrix.of(new double[][]{
			{1.0, -0.5},
			{-1.0, 1.0}});
		var csc = m.compress();

		MKL.solveSparse(
			2,
			csc.values,
			csc.rowIndices,
			csc.columnPointers,
			new double[]{1, 0},
			x
		);

		for (var xi : x) {
			System.out.println(xi);
		}
	}
}
