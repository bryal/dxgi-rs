use super::{ IID_IDXGIFactory1, CreateDXGIFactory1 };
use libc::c_void;
use std::ptr;

#[test]
fn test() {
	let mut factory: *mut c_void = ptr::null_mut();
	assert_eq!(0, unsafe { CreateDXGIFactory1(&IID_IDXGIFactory1, &mut factory) });
}