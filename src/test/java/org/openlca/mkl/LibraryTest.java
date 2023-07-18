package org.openlca.mkl;

import static org.junit.Assert.*;

import org.junit.Test;

public class LibraryTest {

	@Test
	public void testIsLibFolder() {
		assertTrue(MKL.isDefaultLibraryDir());
		assertTrue(MKL.loadFromDefault());
	}

}
