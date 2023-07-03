use jni_sys::{jclass, jdoubleArray, jint, jintArray, JNIEnv};
use std::ffi::{c_char, c_void};
use std::ptr;

mod mkl;

const NULL: *mut u8 = ptr::null_mut();

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_version() -> jint {
  1
}

/// Get the raw pointer of the given array from the JVM.
unsafe fn get_array_f64(env: *mut JNIEnv, array: jdoubleArray) -> *mut f64 {
  return (**env).GetDoubleArrayElements.unwrap()(env, array, NULL);
}

/// Give the data behind the raw pointer of the given array back to the JVM.
unsafe fn release_array_f64(
  env: *mut JNIEnv,
  array: jdoubleArray,
  ptr: *mut f64,
) {
  (**env).ReleaseDoubleArrayElements.unwrap()(env, array, ptr, 0);
}

/// Get the raw pointer of the given array from the JVM.
unsafe fn get_array_i32(env: *mut JNIEnv, array: jintArray) -> *mut i32 {
  return (**env).GetIntArrayElements.unwrap()(env, array, NULL);
}

/// Give the data behind the raw pointer of the given array back to the JVM.
unsafe fn release_array_i32(env: *mut JNIEnv, array: jintArray, ptr: *mut i32) {
  (**env).ReleaseIntArrayElements.unwrap()(env, array, ptr, 0);
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
  unsafe {
    let aPtr = get_array_f64(env, matrix);
    let xPtr = get_array_f64(env, vector);
    let yPtr = get_array_f64(env, result);

    let trans = 'N' as c_char;
    let alpha: f64 = 1.0;
    let beta: f64 = 0.0;
    let inc: i64 = 1;
    let rowsA_64: i64 = rows as i64;
    let colsA_64: i64 = columns as i64;

    mkl::dgemv(
      &trans, &rowsA_64, &colsA_64, &alpha, aPtr, &rowsA_64, xPtr, &inc, &beta,
      yPtr, &inc,
    );

    release_array_f64(env, matrix, aPtr);
    release_array_f64(env, vector, xPtr);
    release_array_f64(env, result, yPtr);
  }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_mkl_MKL_sparseSolve(
  env: *mut JNIEnv,
  _class: jclass,
  n: jint,
  a: jdoubleArray,
  ia: jintArray,
  ja: jintArray,
  b: jdoubleArray,
  x: jdoubleArray,
) {
  unsafe {
    println!("enter func");
    let a_ptr = get_array_f64(env, a);
    let ia_ptr = get_array_i32(env, ia);
    let ja_ptr = get_array_i32(env, ja);
    let b_ptr = get_array_f64(env, b);
    let x_ptr = get_array_f64(env, x);

    let mut pt = vec![0i64; 64];
    let pt_ptr = pt.as_mut_ptr() as *mut c_void;

    let mut perm = vec![0; n as usize];
    let perm_ptr = perm.as_mut_ptr();

    let mut iparm = vec![0; 64];
    let iparm_ptr = iparm.as_mut_ptr();

    let mut error = vec![0; 1];
    let error_ptr = error.as_mut_ptr();

    let maxfct = 1;
    let mnum = 1;
    let mtype = 11;
    let phase = 13;
    let nrhs = 1;
    let msglvl = 1;

    println!("before call");
    mkl::pardiso(
      pt_ptr, &maxfct, &mnum, &mtype, &phase, &n, a_ptr, ia_ptr, ja_ptr,
      perm_ptr, &nrhs, iparm_ptr, &msglvl, b_ptr, x_ptr, error_ptr,
    );
    println!("after call");

    release_array_f64(env, a, a_ptr);
    release_array_i32(env, ia, ia_ptr);
    release_array_i32(env, ja, ja_ptr);
    release_array_f64(env, b, b_ptr);
    release_array_f64(env, x, x_ptr);
    println!("exit func");
  }
}
