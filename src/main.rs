// The MIT License (MIT)
//
// Copyright (c) 2015 Johan Johansson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#![feature(libc)]

extern crate libc;
extern crate winapi;

use libc::c_void;
use winapi::{ REFIID, GUID, HRESULT, ULONG, UINT };


macro_rules! define_guid {
	($name:ident, $d1:expr, $d2:expr, $d3:expr, $d4:expr) => {
		#[allow(non_upper_case_globals, dead_code)]
		static $name: GUID = GUID{ Data1: $d1, Data2: $d2, Data3: $d3, Data4: $d4 };
	}
}

// DEFINE_GUID(IID_IDXGIFactory1,0x770aae78,0xf26f,0x4dba,0xa8,0x29,0x25,0x3c,0x83,0xd1,0xb3,0x87);
define_guid!(IID_IDXGIFactory1,0x770aae78,0xf26f,0x4dba,[0xa8,0x29,0x25,0x3c,0x83,0xd1,0xb3,0x87]);

macro_rules! c_vtable {
	($sn:ident of $sparent:ty {
		$(fn $mn:ident($($argn:ident: $argt:ty),*) -> $rt:ty),*
	}) => {
		struct $sn {
			$(
				$mn: Option<unsafe extern "system" fn(
					this: *mut $sparent, $($argn: $argt),*) -> $rt>
			),*
		}
	}
}

#[repr(C)]
struct IDXGIFactory {
	vtable: *mut IDXGIFactoryVtbl
}

#[repr(C)] c_vtable!(
IDXGIFactoryVtbl of IDXGIFactory {
	fn QueryInterface(riid: REFIID, object: *mut *mut c_void) -> HRESULT,

	fn AddRef() -> ULONG,

	fn Release() -> ULONG,

	fn SetPrivateData(name: REFGUID, data_size: UINT, data: *const c_void) -> HRESULT,

	fn SetPrivateDataInterface(name: REFGUID, unknown: *const IUnknown) -> HRESULT,

	fn GetPrivateData(name: REFGUID, data_size: UINT, data: *mut c_void) -> HRESULT,

	fn GetParent(riid: REFIID, parent: *mut *mut c_void) -> HRESULT,

	fn EnumAdapters(adapter_i: UINT, out_adapter: *mut *mut IDXGIAdapter) -> HRESULT,

	fn MakeWindowAssociation(window_handle: HWND, flags: UINT) -> HRESULT,

	fn GetWindowAssociation(out_window_handle: *mut HWND) -> HRESULT,

	fn CreateSwapChain(device: *mut IUnknown, desc: *mut DXGI_SWAP_CHAIN_DESC,
		out_swapchain: *mut *mut IDXGISwapChain) -> HRESULT,

	fn CreateSoftwareAdapter(module: HMODULE, out_adapter: *mut *mut IDXGIAdapter) -> HRESULT
});

#[link(name="dxgi")]
extern "C" {
	#![allow(dead_code)]

	fn CreateDXGIFactory1(riid: REFIID, ppFactory: *mut *mut c_void) -> HRESULT;
}

fn main() {
	use std::ptr;
	let mut factory: *mut c_void = ptr::null_mut();
	assert_eq!(0, unsafe { CreateDXGIFactory1(&IID_IDXGIFactory1, &mut factory) });
}