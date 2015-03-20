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

#![allow(non_snake_case, dead_code, non_upper_case_globals, non_camel_case_types)]

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

define_guid!(IID_IDXGIDisplayControl,
	0xea9dbf1a, 0xc88e, 0x4486, [0x85, 0x4a, 0x98, 0xaa, 0x01, 0x38, 0xf3, 0x0c]);
define_guid!(IID_IDXGIOutputDuplication,
	0x191cfac3, 0xa341, 0x470d, [0xb2, 0x6e, 0xa8, 0x64, 0xf4, 0x28, 0x31, 0x9c]);
define_guid!(IID_IDXGISurface2,
	0xaba496dd, 0xb617, 0x4cb8, [0xa8, 0x66, 0xbc, 0x44, 0xd7, 0xeb, 0x1f, 0xa2]);
define_guid!(IID_IDXGIResource1,
	0x30961379, 0x4609, 0x4a41, [0x99, 0x8e, 0x54, 0xfe, 0x56, 0x7e, 0xe0, 0xc1]);
define_guid!(IID_IDXGIDevice2,
	0x05008617, 0xfbfd, 0x4051, [0xa7, 0x90, 0x14, 0x48, 0x84, 0xb4, 0xf6, 0xa9]);
define_guid!(IID_IDXGISwapChain1,
	0x790a45f7, 0x0d42, 0x4876, [0x98, 0x3a, 0x0a, 0x55, 0xcf, 0xe6, 0xf4, 0xaa]);
define_guid!(IID_IDXGIFactory2,
	0x50c83a1c, 0xe072, 0x4c48, [0x87, 0xb0, 0x36, 0x30, 0xfa, 0x36, 0xa6, 0xd0]);
define_guid!(IID_IDXGIAdapter2,
	0x0AA1AE0A, 0xFA0E, 0x4B84, [0x86, 0x44, 0xE0, 0x5F, 0xF8, 0xE5, 0xAC, 0xB5]);
define_guid!(IID_IDXGIOutput1,
	0x00cddea8, 0x939b, 0x4b83, [0xa3, 0x40, 0xa6, 0x85, 0x22, 0x66, 0x66, 0xcc]);

define_guid!(IID_IDXGIDevice3,
	0x6007896c, 0x3244, 0x4afd, [0xbf, 0x18, 0xa6, 0xd3, 0xbe, 0xda, 0x50, 0x23]);
define_guid!(IID_IDXGISwapChain2,
	0xa8be2ac4, 0x199f, 0x4946, [0xb3, 0x31, 0x79, 0x59, 0x9f, 0xb9, 0x8d, 0xe7]);
define_guid!(IID_IDXGIOutput2,
	0x595e39d1, 0x2724, 0x4663, [0x99, 0xb1, 0xda, 0x96, 0x9d, 0xe2, 0x83, 0x64]);
define_guid!(IID_IDXGIFactory3,
	0x25483823, 0xcd46, 0x4c7d, [0x86, 0xca, 0x47, 0xaa, 0x95, 0xb8, 0x37, 0xbd]);
define_guid!(IID_IDXGIDecodeSwapChain,
	0x2633066b, 0x4514, 0x4c7a, [0x8f, 0xd8, 0x12, 0xea, 0x98, 0x05, 0x9d, 0x18]);
define_guid!(IID_IDXGIFactoryMedia,
	0x41e7d1f2, 0xa591, 0x4f7b, [0xa2, 0xe5, 0xfa, 0x9c, 0x84, 0x3e, 0x1c, 0x12]);
define_guid!(IID_IDXGISwapChainMedia,
	0xdd95b90b, 0xf05f, 0x4f6a, [0xbd, 0x65, 0x25, 0xbf, 0xb2, 0x64, 0xbd, 0x84]);
define_guid!(IID_IDXGIOutput3,
	0x8a6bb301, 0x7e7e, 0x41F4, [0xa8, 0xe0, 0x5b, 0x32, 0xf7, 0xf9, 0x9b, 0x18]);



pub const DXGI_ENUM_MODES_INTERLACED: ULONG = 1;
pub const DXGI_ENUM_MODES_SCALING: ULONG = 2;
pub const DXGI_ENUM_MODES_STEREO: ULONG = 4;
pub const DXGI_ENUM_MODES_DISABLED_STEREO: ULONG = 8;

pub const DXGI_PRESENT_DO_NOT_SEQUENCE: ULONG = 0x00000002;
pub const DXGI_PRESENT_TEST: ULONG = 0x00000001;
pub const DXGI_PRESENT_RESTART: ULONG = 0x00000004;
pub const DXGI_PRESENT_DO_NOT_WAIT: ULONG = 0x00000008;
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: ULONG = 0x00000010;
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: ULONG = 0x00000020;
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: ULONG = 0x00000040;
pub const DXGI_PRESENT_USE_DURATION: ULONG = 0x00000100;

pub type DXGI_USAGE = UINT;
pub const DXGI_USAGE_BACK_BUFFER: DXGI_USAGE = 1 << (2 + 4);
pub const DXGI_USAGE_DISCARD_ON_PRESENT: DXGI_USAGE = 1 << (5 + 4);
pub const DXGI_USAGE_READ_ONLY: DXGI_USAGE = 1 << (4 + 4);
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: DXGI_USAGE = 1 << (1 + 4);
pub const DXGI_USAGE_SHADER_INPUT: DXGI_USAGE = 1 << (0 + 4);
pub const DXGI_USAGE_SHARED: DXGI_USAGE = 1 << (3 + 4);
pub const DXGI_USAGE_UNORDERED_ACCESS: DXGI_USAGE = 1 << (6 + 4);