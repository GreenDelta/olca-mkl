package org.openlca.mkl;

import org.openlca.core.matrix.solvers.Factorization;

import java.util.Arrays;
import java.util.concurrent.atomic.AtomicBoolean;

class DenseFactorization implements Factorization {

	private final long pointer;
	private final int size;
	private final AtomicBoolean isDisposed;

	DenseFactorization(long pointer, int size) {
		this.pointer = pointer;
		this.size = size;
		this.isDisposed = new AtomicBoolean(false);
	}

	@Override
	public int size() {
		return size;
	}

	@Override
	public double[] solve(double[] b) {
		if (b == null)
			return new double[size];
		var x = Arrays.copyOf(b, b.length);
		int error = MKL.solveDenseFactorization(pointer, 1, x);
		// TODO: translate MKL errors to Apache Math
		if (error != 0)
			throw new RuntimeException("MKL-Error: " + error);
		return x;
	}

	@Override
	public void dispose() {
		if (isDisposed.get())
			return;
		synchronized (isDisposed) {
			if (isDisposed.get())
				return;
			MKL.disposeDenseFactorization(pointer);
		}
	}

	@Override
	public boolean isDisposed() {
		return isDisposed.get();
	}
}
