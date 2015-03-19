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
// use winapi::{ REFIID };//, GUID };

type HResult = u32;

#[repr(C)]
struct GUID {
	Data1: u32,
	Data2: u16,
	Data3: u16,
	Data4: [u8; 8],
}
type REFIID = *const GUID;

macro_rules! define_guid {
	($name:ident, $d1:expr, $d2:expr, $d3:expr, $d4:expr) => {
		#[allow(non_upper_case_globals, dead_code)]
		static $name: GUID = GUID{ Data1: $d1, Data2: $d2, Data3: $d3, Data4: $d4 };
	}
}

// DEFINE_GUID(IID_IDXGIFactory1,0x770aae78,0xf26f,0x4dba,0xa8,0x29,0x25,0x3c,0x83,0xd1,0xb3,0x87);
define_guid!(IID_IDXGIFactory1,0x770aae78,0xf26f,0x4dba,[0xa8,0x29,0x25,0x3c,0x83,0xd1,0xb3,0x87]);

// DEFINE_GUID(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8)
// static IID_IDXGIFactory1: GUID = githgitGUID{ Data1: l, w1, w2, { b1, b2,  b3,  b4,  b5,  b6,  b7,  b8 } };

#[link(name="dxgi")]
extern "C" {
	#![allow(dead_code)]

	fn CreateDXGIFactory1(riid: REFIID, ppFactory: *mut *mut c_void) -> HResult;
}

#[test]
fn test() {
	use std::ptr;
	let mut p: *mut c_void = ptr::null_mut();
	println!("p: {}", unsafe { CreateDXGIFactory1(&IID_IDXGIFactory1, &mut p) });
}