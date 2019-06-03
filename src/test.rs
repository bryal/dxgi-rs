use { IDXGIFactory1, IID_IDXGIFactory1, CreateDXGIFactory1 };
use std::ptr;

#[test]
fn test() {
	unsafe {
		let factory = {
			let mut factory: *mut _ = ptr::null_mut();
			assert_eq!(0, CreateDXGIFactory1(&IID_IDXGIFactory1, &mut factory));
			factory as *mut IDXGIFactory1 };

		assert!(factory as usize != 0);

		println!("IsCurrent: {}", (*factory).IsCurrent() != 0);

		assert_eq!((*factory).AddRef(), 2);

		assert_eq!((*factory).Release(), 1);
	}
}
