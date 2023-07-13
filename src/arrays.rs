use jni_sys::{jdoubleArray, jintArray, jlongArray, JNIEnv};
use std::ptr;

const NULL: *mut u8 = ptr::null_mut();

#[inline]
pub unsafe fn get_f64(env: *mut JNIEnv, array: jdoubleArray) -> *mut f64 {
  return (**env).GetDoubleArrayElements.unwrap()(env, array, NULL);
}

#[inline]
pub unsafe fn drop_f64(
  env: *mut JNIEnv,
  array: jdoubleArray,
  ptr: *mut f64,
) {
  (**env).ReleaseDoubleArrayElements.unwrap()(env, array, ptr, 0);
}

#[inline]
pub unsafe fn get_i32(env: *mut JNIEnv, array: jintArray) -> *mut i32 {
  return (**env).GetIntArrayElements.unwrap()(env, array, NULL);
}

#[inline]
pub unsafe fn drop_i32(
  env: *mut JNIEnv,
  array: jintArray,
  ptr: *mut i32,
) {
  (**env).ReleaseIntArrayElements.unwrap()(env, array, ptr, 0);
}

#[inline]
pub unsafe fn get_i64(env: *mut JNIEnv, array: jlongArray) -> *mut i64 {
  return (**env).GetLongArrayElements.unwrap()(env, array, NULL);
}

#[inline]
pub unsafe fn drop_i64(
  env: *mut JNIEnv,
  array: jlongArray,
  ptr: *mut i64,
) {
  (**env).ReleaseLongArrayElements.unwrap()(env, array, ptr, 0);
}
