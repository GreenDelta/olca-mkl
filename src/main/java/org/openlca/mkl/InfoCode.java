package org.openlca.mkl;

class InfoCode {

	private InfoCode() {
	}

	static void checkPardiso(int code) {
		if (code == 0)
			return;
		// TODO: translate Pardise error codes
		throw new RuntimeException("Pardiso error: code = " + code);
	}

}
