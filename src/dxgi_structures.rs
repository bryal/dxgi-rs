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

//! Structures provided by DXGI. Structures are from DXGI 1.0, 1.1, and 1.2
//!
//! # References
//! [DXGI Structures, MSDN](https://msdn.microsoft.com/en-us/library/windows/desktop/ff471323(v=vs.85).aspx)

use winapi::{ WCHAR, UINT, SIZE_T, LUID, LARGE_INTEGER,
	BOOL, FLOAT, RECT, HMONITOR, POINT, HANDLE };
use libc::{ c_char, c_float };

use dxgi_constants::*;

#[repr(C)] pub struct DXGI_ADAPTER_DESC {
	Description: [WCHAR; 128],
	VendorId: UINT,
	DeviceId: UINT,
	SubSysId: UINT,
	Revision: UINT,
	DedicatedVideoMemory: SIZE_T,
	DedicatedSystemMemory: SIZE_T,
	SharedSystemMemory: SIZE_T,
	AdapterLuid: LUID,
}

#[repr(C)] pub struct DXGI_ADAPTER_DESC1 {
	Description: [WCHAR; 128],
	VendorId: UINT,
	DeviceId: UINT,
	SubSysId: UINT,
	Revision: UINT,
	DedicatedVideoMemory: SIZE_T,
	DedicatedSystemMemory: SIZE_T,
	SharedSystemMemory: SIZE_T,
	AdapterLuid: LUID,
	Flags: UINT,
}

#[repr(C)] pub struct DXGI_ADAPTER_DESC2 {
	Description: [WCHAR; 128],
	VendorId: UINT,
	DeviceId: UINT,
	SubSysId: UINT,
	Revision: UINT,
	DedicatedVideoMemory: SIZE_T,
	DedicatedSystemMemory: SIZE_T,
	SharedSystemMemory: SIZE_T,
	AdapterLuid: LUID,
	Flags: UINT,
	GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
	ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}

#[repr(C)] pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
	Flags: UINT,
}

#[repr(C)] pub struct DXGI_FRAME_STATISTICS {
	PresentCount: UINT,
	PresentRefreshCount: UINT,
	SyncRefreshCount: UINT,
	SyncQPCTime: LARGE_INTEGER,
	SyncGPUTime: LARGE_INTEGER,
}

#[repr(C)] pub struct DXGI_GAMMA_CONTROL {
	Scale: DXGI_RGB,
	Offset: DXGI_RGB,
	GammaCurve: [DXGI_RGB; 1025],
}

#[repr(C)] pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
	ScaleAndOffsetSupported: BOOL,
	MaxConvertedValue: c_float,
	MinConvertedValue: c_float,
	NumGammaControlPoints: UINT,
	ControlPointPositions: [c_float; 1025],
}

#[repr(C)] pub struct DXGI_INFO_QUEUE_FILTER {
	AllowList: DXGI_INFO_QUEUE_FILTER_DESC,
	DenyList: DXGI_INFO_QUEUE_FILTER_DESC,
}

#[repr(C)] pub struct DXGI_INFO_QUEUE_FILTER_DESC {
	NumCategories: UINT,
	pCategoryList: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
	NumSeverities: UINT,
	pSeverityList: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
	NumIDs: UINT,
	pIDList: DXGI_INFO_QUEUE_MESSAGE_ID,
}

#[repr(C)] pub struct DXGI_INFO_QUEUE_MESSAGE {
	Producer: DXGI_DEBUG_ID,
	Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
	Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
	ID: DXGI_INFO_QUEUE_MESSAGE_ID,
	pDescription: *const c_char,
	DescriptionByteLength: SIZE_T,
}

#[repr(C)] pub struct _DXGI_MATRIX_3X2_F {
	_11: FLOAT,
	_12: FLOAT,
	_21: FLOAT,
	_22: FLOAT,
	_31: FLOAT,
	_32: FLOAT,
}

#[repr(C)] pub struct DXGI_MAPPED_RECT {
	Pitch: INT,
	pBits: BYTE,
}

#[repr(C)] pub struct DXGI_MODE_DESC {
	Width: UINT,
	Height: UINT,
	RefreshRate: DXGI_RATIONAL,
	Format: DXGI_FORMAT,
	ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
	Scaling: DXGI_MODE_SCALING,
}

#[repr(C)] pub struct _DXGI_MODE_DESC1 {
	Width: UINT,
	Height: UINT,
	RefreshRate: DXGI_RATIONAL,
	Format: DXGI_FORMAT,
	ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
	Scaling: DXGI_MODE_SCALING,
	Stereo: BOOL,
}

#[repr(C)] pub struct DXGI_OUTPUT_DESC {
	DeviceName: [WCHAR; 32],
	DesktopCoordinates: RECT,
	AttachedToDesktop: BOOL,
	Rotation: DXGI_MODE_ROTATION,
	Monitor: HMONITOR,
}

#[repr(C)] pub struct _DXGI_OUTDUPL_DESC {
	ModeDesc: DXGI_MODE_DESC,
	Rotation: DXGI_MODE_ROTATION,
	DesktopImageInSystemMemory: BOOL,
}

#[repr(C)] pub struct _DXGI_OUTDUPL_FRAME_INFO {
	LastPresentTime: LARGE_INTEGER,
	LastMouseUpdateTime: LARGE_INTEGER,
	AccumulatedFrames: UINT,
	RectsCoalesced: BOOL,
	ProtectedContentMaskedOut: BOOL,
	PointerPosition: DXGI_OUTDUPL_POINTER_POSITION,
	TotalMetadataBufferSize: UINT,
	PointerShapeBufferSize: UINT,
}

#[repr(C)] pub struct _DXGI_OUTDUPL_MOVE_RECT {
	SourcePoint: POINT,
	DestinationRect: RECT,
}

#[repr(C)] pub struct _DXGI_OUTDUPL_POINTER_POSITION {
	Position: POINT,
	Visible: BOOL,
}

#[repr(C)] pub struct _DXGI_OUTDUPL_POINTER_SHAPE_INFO {
	Type: UINT,
	Width: UINT,
	Height: UINT,
	Pitch: UINT,
	HotSpot: POINT,
}

#[repr(C)] pub struct DXGI_PRESENT_PARAMETERS {
	DirtyRectsCount: UINT,
	pDirtyRects: RECT,
	pScrollRect: RECT,
	pScrollOffset: POINT,
}

#[repr(C)] pub struct DXGI_RATIONAL {
	Numerator: UINT,
	Denominator: UINT,
}

#[repr(C)] pub struct DXGI_RGB {
	Red: c_float,
	Green: c_float,
	Blue: c_float,
}

#[repr(C)] pub struct DXGI_RGBA {
	r: c_float,
	g: c_float,
	b: c_float,
	a: c_float,
}

#[repr(C)] pub struct DXGI_SAMPLE_DESC {
	Count: UINT,
	Quality: UINT,
}

#[repr(C)] pub struct DXGI_SHARED_RESOURCE {
	Handle: HANDLE,
}

#[repr(C)] pub struct DXGI_SURFACE_DESC {
	Width: UINT,
	Height: UINT,
	Format: DXGI_FORMAT,
	SampleDesc: DXGI_SAMPLE_DESC,
}

#[repr(C)] pub struct DXGI_SWAP_CHAIN_DESC {
	BufferDesc: DXGI_MODE_DESC,
	SampleDesc: DXGI_SAMPLE_DESC,
	BufferUsage: DXGI_USAGE,
	BufferCount: UINT,
	OutputWindow: HWND,
	Windowed: BOOL,
	SwapEffect: DXGI_SWAP_EFFECT,
	Flags: UINT,
}

#[repr(C)] pub struct _DXGI_SWAP_CHAIN_DESC1 {
	Width: UINT,
	Height: UINT,
	Format: DXGI_FORMAT,
	Stereo: BOOL,
	SampleDesc: DXGI_SAMPLE_DESC,
	BufferUsage: DXGI_USAGE,
	BufferCount: UINT,
	Scaling: DXGI_SCALING,
	SwapEffect: DXGI_SWAP_EFFECT,
	AlphaMode: DXGI_ALPHA_MODE,
	Flags: UINT,
}

#[repr(C)] pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
	RefreshRate: DXGI_RATIONAL,
	ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
	Scaling: DXGI_MODE_SCALING,
	Windowed: BOOL,
}