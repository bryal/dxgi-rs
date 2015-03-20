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
use winapi::{ REFIID, HRESULT, GUID };
use dxgi_structures::*;

mod macros;
mod dxgi_structures;
mod dxgi_enumerations;
mod dxgi_constants;
mod dxgi_interfaces;


define_guid!(IID_IDXGIObject,
	0xaec22fb8, 0x76f3, 0x4639, [0x9b, 0xe0, 0x28, 0xeb, 0x43, 0xa6, 0x7a, 0x2e]);
define_guid!(IID_IDXGIDeviceSubObject,
	0x3d3e0379, 0xf9de, 0x4d58, [0xbb, 0x6c, 0x18, 0xd6, 0x29, 0x92, 0xf1, 0xa6]);
define_guid!(IID_IDXGIResource,
	0x035f3ab4, 0x482e, 0x4e50, [0xb4, 0x1f, 0x8a, 0x7f, 0x8b, 0xd8, 0x96, 0x0b]);
define_guid!(IID_IDXGIKeyedMutex,
	0x9d8e1289, 0xd7b3, 0x465f, [0x81, 0x26, 0x25, 0x0e, 0x34, 0x9a, 0xf8, 0x5d]);
define_guid!(IID_IDXGISurface,
	0xcafcb56c, 0x6ac3, 0x4889, [0xbf, 0x47, 0x9e, 0x23, 0xbb, 0xd2, 0x60, 0xec]);
define_guid!(IID_IDXGISurface1,
	0x4AE63092, 0x6327, 0x4c1b, [0x80, 0xAE, 0xBF, 0xE1, 0x2E, 0xA3, 0x2B, 0x86]);
define_guid!(IID_IDXGIAdapter,
	0x2411e7e1, 0x12ac, 0x4ccf, [0xbd, 0x14, 0x97, 0x98, 0xe8, 0x53, 0x4d, 0xc0]);
define_guid!(IID_IDXGIOutput,
	0xae02eedb, 0xc735, 0x4690, [0x8d, 0x52, 0x5a, 0x8d, 0xc2, 0x02, 0x13, 0xaa]);
define_guid!(IID_IDXGISwapChain,
	0x310d36a0, 0xd2e7, 0x4c0a, [0xaa, 0x04, 0x6a, 0x9d, 0x23, 0xb8, 0x88, 0x6a]);
define_guid!(IID_IDXGIFactory,
	0x7b7166ec, 0x21c7, 0x44ae, [0xb2, 0x1a, 0xc9, 0xae, 0x32, 0x1a, 0xe3, 0x69]);
define_guid!(IID_IDXGIDevice,
	0x54ec77fa, 0x1377, 0x44e6, [0x8c, 0x32, 0x88, 0xfd, 0x5f, 0x44, 0xc8, 0x4c]);
define_guid!(IID_IDXGIFactory1,
	0x770aae78, 0xf26f, 0x4dba, [0xa8, 0x29, 0x25, 0x3c, 0x83, 0xd1, 0xb3, 0x87]);
define_guid!(IID_IDXGIAdapter1,
	0x29038f61, 0x3839, 0x4626, [0x91, 0xfd, 0x08, 0x68, 0x79, 0x01, 0x1a, 0x05]);
define_guid!(IID_IDXGIDevice1,
	0x77db970f, 0x6276, 0x48ba, [0xba, 0x28, 0x07, 0x01, 0x43, 0xb4, 0x39, 0x2c]);

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