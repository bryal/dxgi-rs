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
//!
//! # References
//! [DXGI Constants, MSDN](https://msdn.microsoft.com/en-us/library/windows/desktop/ff471319(v=vs.85).aspx)

use winapi::{ GUID, ULONG, UINT };
pub use winapi::winerror::{ DXGI_ERROR_DEVICE_HUNG, DXGI_ERROR_DEVICE_REMOVED,
	DXGI_ERROR_DEVICE_RESET, DXGI_ERROR_DRIVER_INTERNAL_ERROR,
	DXGI_ERROR_FRAME_STATISTICS_DISJOINT, DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE,
	DXGI_ERROR_INVALID_CALL, DXGI_ERROR_MORE_DATA, DXGI_ERROR_NONEXCLUSIVE,
	DXGI_ERROR_NOT_CURRENTLY_AVAILABLE, DXGI_ERROR_NOT_FOUND,
	DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED, DXGI_ERROR_REMOTE_OUTOFMEMORY,
	DXGI_ERROR_WAS_STILL_DRAWING, DXGI_ERROR_UNSUPPORTED, DXGI_ERROR_ACCESS_LOST,
	DXGI_ERROR_WAIT_TIMEOUT, DXGI_ERROR_SESSION_DISCONNECTED,
	DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE, DXGI_ERROR_CANNOT_PROTECT_CONTENT,
	DXGI_ERROR_ACCESS_DENIED, DXGI_ERROR_NAME_ALREADY_EXISTS, S_OK };
pub use winapi::winerror::{ DXGI_STATUS_OCCLUDED, DXGI_STATUS_MODE_CHANGED,
	DXGI_STATUS_MODE_CHANGE_IN_PROGRESS };


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

pub static DXGI_ENUM_MODES_INTERLACED: ULONG = 1;
pub static DXGI_ENUM_MODES_SCALING: ULONG = 2;
pub static DXGI_ENUM_MODES_STEREO: ULONG = 4;
pub static DXGI_ENUM_MODES_DISABLED_STEREO: ULONG = 8;

pub static DXGI_PRESENT_DO_NOT_SEQUENCE: ULONG = 0x00000002;
pub static DXGI_PRESENT_TEST: ULONG = 0x00000001;
pub static DXGI_PRESENT_RESTART: ULONG = 0x00000004;
pub static DXGI_PRESENT_DO_NOT_WAIT: ULONG = 0x00000008;
pub static DXGI_PRESENT_RESTRICT_TO_OUTPUT: ULONG = 0x00000010;
pub static DXGI_PRESENT_STEREO_PREFER_RIGHT: ULONG = 0x00000020;
pub static DXGI_PRESENT_STEREO_TEMPORARY_MONO: ULONG = 0x00000040;
pub static DXGI_PRESENT_USE_DURATION: ULONG = 0x00000100;

pub type DXGI_USAGE = UINT;
pub static DXGI_USAGE_BACK_BUFFER: DXGI_USAGE = 1 << (2 + 4);
pub static DXGI_USAGE_DISCARD_ON_PRESENT: DXGI_USAGE = 1 << (5 + 4);
pub static DXGI_USAGE_READ_ONLY: DXGI_USAGE = 1 << (4 + 4);
pub static DXGI_USAGE_RENDER_TARGET_OUTPUT: DXGI_USAGE = 1 << (1 + 4);
pub static DXGI_USAGE_SHADER_INPUT: DXGI_USAGE = 1 << (0 + 4);
pub static DXGI_USAGE_SHARED: DXGI_USAGE = 1 << (3 + 4);
pub static DXGI_USAGE_UNORDERED_ACCESS: DXGI_USAGE = 1 << (6 + 4);