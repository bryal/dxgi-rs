use super::{ IDXGIFactory1, IID_IDXGIFactory1, CreateDXGIFactory1 };
use libc::c_void;
use std::ptr;

#[test]
fn test() {
	unsafe {

	let factory = {
		let mut factory: *mut c_void = ptr::null_mut();
		assert_eq!(0, CreateDXGIFactory1(&IID_IDXGIFactory1, &mut factory));
		factory as *mut IDXGIFactory1 };

	assert!(factory as usize != 0);

	println!("IsCurrent: {}", c_mtdcall!(factory->IsCurrent()) != 0);

	}
}