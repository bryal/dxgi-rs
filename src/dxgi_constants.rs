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

//! Constants and related typedefs provided by DXGI

use winapi::{ GUID };


pub type DXGI_DEBUG_ID = GUID;

define_guid!(DXGI_DEBUG_ALL,
	0xe48ae283, 0xda80, 0x490b,	[0x87, 0xe6, 0x43, 0xe9, 0xa9, 0xcf, 0xda, 0x8]);
define_guid!(DXGI_DEBUG_DX,
	0x35cdd7fc, 0x13b2, 0x421d,	[0xa5, 0xd7, 0x7e, 0x44, 0x51, 0x28, 0x7d, 0x64]);
define_guid!(DXGI_DEBUG_DXGI,
	0x25cddaa4, 0xb1c6, 0x47e1,	[0xac, 0x3e, 0x98, 0x87, 0x5b, 0x5a, 0x2e, 0x2a]);
define_guid!(DXGI_DEBUG_APP,
	0x6cd6e01, 0x4219, 0x4ebd,	[0x87, 0x9, 0x27, 0xed, 0x23, 0x36, 0xc, 0x62]);
define_guid!(DXGI_DEBUG_D3D11,
	0x4b99317b, 0xac39, 0x4aa6,	[0xbb, 0xb, 0xba, 0xa0, 0x47, 0x84, 0x79, 0x8f]);