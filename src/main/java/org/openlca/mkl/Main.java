package org.openlca.mkl;

import java.io.File;

public class Main {

	public static void main(String[] args) {
		var loaded = MKL.loadFrom(new File("target/release"));
		if (!loaded) {
			System.out.println("failed to load kernel");
			return;
		}
		System.out.println(MKL.version());
	}
}
