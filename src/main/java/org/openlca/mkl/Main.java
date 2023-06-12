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
	}
}
