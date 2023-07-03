package org.openlca.mkl;

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
		MKL.denseMatrixVectorMultiplication(
			3, 3,
			new double[]{1, 2, 3, 4, 5, 6, 7, 8, 9},
			new double[]{1, 2, 3},
			y);
		for (double yi : y) {
			System.out.println(yi);
		}

		double[] x = new double[8];
		double[] a = new double[]{1.0, -1.0, -3.0, -2.0, 5.0, 4.0, 6.0, 4.0, -4.0, 2.0, 7.0, 8.0, -5.0};
		int[] ia = new int[]{1, 4, 6, 9, 12, 14};
		int[] ja = new int[]{1, 2, 4, 1, 2, 3, 4, 5, 1, 3, 4, 2, 5};

		for (int i = 0; i < ia.length; i++) {
			ia[i] -= 1;
		}
		for (int i = 0; i < ja.length; i++) {
			ja[i] -= 1;
		}

		MKL.sparseSolve(
			5,
			a,
			ia,
			ja,
			new double[]{1, 1, 1, 1, 1},
			x
		);
	}
}
