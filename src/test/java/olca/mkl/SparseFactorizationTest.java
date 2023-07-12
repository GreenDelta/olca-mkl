package olca.mkl;

import static org.junit.Assert.*;

import org.junit.BeforeClass;
import org.junit.Test;
import org.openlca.core.matrix.format.HashPointMatrix;
import org.openlca.mkl.MKL;
import org.openlca.mkl.MKLSolver;

import java.io.File;

public class SparseFactorizationTest {

	@BeforeClass
	public static void setup() {
		MKL.loadFrom(new File("bin")); // TODO
	}


	@Test
	public void testSimple() {
		var m = HashPointMatrix.of(new double[][] {
			{1, -0.5},
			{-1, 1.0}
		});
		var f = new MKLSolver().factorize(m);
		var x = f.solve(new double[] {1.0, 0.0});
		f.dispose();

		assertEquals(2.0, x[0], 1e-16);
		assertEquals(2.0, x[1], 1e-16);
	}
}
