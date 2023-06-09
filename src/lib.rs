use jni_sys::{jclass, jdoubleArray, jint, JNIEnv};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_version() -> jint {
  1
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_denseMatrixVectorMultiplication(
  env: *mut JNIEnv,
  _class: jclass,
  rows: jint,
  columns: jint,
  matrix: jdoubleArray,
  vector: jdoubleArray,
  result: jdoubleArray,
) {
  print!("called mv-mul")
}
