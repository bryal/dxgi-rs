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

//! Interfaces provided by DXGI.
//!
//! # References
//! [DXGI Interfaces, MSDN]
//! (https://msdn.microsoft.com/en-us/library/windows/desktop/ff471322(v=vs.85).aspx)

#![allow(non_snake_case, dead_code)]

use winapi::{ HRESULT, REFGUID, UINT, LPCWSTR, REFIID, LARGE_INTEGER, GUID, HWND, HMODULE, BOOL,
	INT, HANDLE, RECT, LUID, DWORD, UINT64, LPCSTR, SIZE_T, SECURITY_ATTRIBUTES, c_void };
pub use winapi::{ IUnknown, IUnknownVtbl };

use structures::*;
use enumerations::*;
use constants::*;

/// Basic COM object functionality
pub trait COMInterface {
	fn i_unknown(&self) -> &IUnknown;
	fn i_unknown_mut(&mut self) -> &mut IUnknown;
}

impl COMInterface for IUnknown {
	fn i_unknown(&self) -> &IUnknown { self }
	fn i_unknown_mut(&mut self) -> &mut IUnknown { self }
}

com_interface!{ IDXGIObject(IDXGIObjectVtbl): IUnknown(IUnknownVtbl) {
	fn SetPrivateData(&mut self, name: REFGUID, data_size: UINT, data: *const c_void) -> HRESULT,
	fn SetPrivateDataInterface(&mut self, name: REFGUID, unknown: *const IUnknown) -> HRESULT,
	fn GetPrivateData(&mut self, name: REFGUID, data_size: UINT, data: *mut c_void) -> HRESULT,
	fn GetParent(&mut self, riid: REFIID, parent: *mut *mut c_void) -> HRESULT
}}
com_interface!{ IDXGIAdapter(IDXGIAdapterVtbl): IDXGIObject(IDXGIObjectVtbl) {
	fn EnumOutputs(&mut self, output_i: UINT, output:*mut *mut IDXGIOutput) -> HRESULT,
	fn GetDesc(&mut self, desc: *mut *mut DXGI_ADAPTER_DESC) -> HRESULT,
	fn CheckInterfaceSupport(&mut self, interface_name: REFGUID, umd_version: LARGE_INTEGER) -> HRESULT
}}
com_interface!{ IDXGIAdapter1(IDXGIAdapter1Vtbl): IDXGIAdapter(IDXGIAdapterVtbl) {
	fn GetDesc1(&mut self, desc: *mut DXGI_ADAPTER_DESC1) -> HRESULT
}}
com_interface!{ IDXGIAdapter2(IDXGIAdapter2Vtbl): IDXGIAdapter(IDXGIAdapterVtbl) {
	fn GetDesc2(&mut self, desc: *mut DXGI_ADAPTER_DESC2) -> HRESULT
}}
com_interface!{ IDXGIDevice(IDXGIDeviceVtbl): IDXGIObject(IDXGIObjectVtbl) {
	fn GetAdapter(&mut self, adapter: *mut *mut IDXGIAdapter) -> HRESULT,
	fn CreateSurface(&mut self, desc: *const DXGI_SURFACE_DESC, num_surfaces: UINT, usage: DXGI_USAGE, shared_resource: *const DXGI_SHARED_RESOURCE, surface: *mut *mut IDXGISurface) -> HRESULT,
	fn QueryResourceResidency(&mut self, resources: *const IUnknown, residency_status: *mut DXGI_RESIDENCY, num_resources: UINT) -> HRESULT,
	fn SetGPUThreadPriority(&mut self, priority: INT) -> HRESULT,
	fn GetGPUThreadPriority(&mut self, priority: *mut INT) -> HRESULT
}}
com_interface!{ IDXGIDevice1(IDXGIDevice1Vtbl): IDXGIDevice(IDXGIDeviceVtbl) {
	fn GetMaximumFrameLatency(&mut self, max_latency: *mut UINT) -> HRESULT,
	fn SetMaximumFrameLatency(&mut self, max_latency: UINT) -> HRESULT
}}
com_interface!{ IDXGIDevice2(IDXGIDevice2Vtbl): IDXGIDevice1(IDXGIDevice1Vtbl) {
	fn OfferResources(&mut self, num_resources: UINT, resources: *const *mut IDXGIResource, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> HRESULT,
	fn ReclaimResources(&mut self, num_resources: UINT, resources: *const *mut IDXGIResource, discarded: *mut BOOL) -> HRESULT,
	fn EnqueueSetEvent(&mut self, event: HANDLE) -> HRESULT
}}
com_interface!{ IDXGIDevice3(IDXGIDevice3Vtbl): IDXGIDevice2(IDXGIDevice2Vtbl) {
	fn Trim(&mut self) -> ()
}}
com_interface!{ IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl): IDXGIObject(IDXGIObjectVtbl) {
	fn GetDevice(&mut self, riid: REFIID, device: *mut *mut c_void) -> HRESULT
}}
com_interface!{ IDXGIKeyedMutex(IDXGIKeyedMutexVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
	fn AcquireSync(&mut self, key: UINT64, milliseconds: DWORD) -> HRESULT,
	fn ReleaseSync(&mut self, key: UINT64) -> HRESULT
}}
com_interface!{ IDXGIResource(IDXGIResourceVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
	fn GetSharedHandle(&mut self, shared_handle: *mut HANDLE) -> HRESULT,
	fn GetUsage(&mut self, usage: *mut DXGI_USAGE) -> HRESULT,
	fn SetEvictionPriority(&mut self, eviction_priority: UINT) -> HRESULT,
	fn GetEvictionPriority(&mut self, evition_priority: *mut UINT) -> HRESULT
}}
com_interface!{ IDXGIResource1(IDXGIResource1Vtbl): IDXGIResource(IDXGIResourceVtbl) {
	fn CreateSubresourceSurface(&mut self, index: UINT, surface: *mut *mut IDXGISurface2) -> HRESULT,
	fn CreateSharedHandle(&mut self, attributes: *const SECURITY_ATTRIBUTES, access: DWORD, name: LPCWSTR, handle: *mut HANDLE) -> HRESULT
}}
com_interface!{ IDXGISurface(IDXGISurfaceVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
	fn GetDesc(&mut self, desc: *mut DXGI_SURFACE_DESC) -> HRESULT,
	fn Map(&mut self, locked_rect: *mut DXGI_MAPPED_RECT, map_flags: UINT) -> HRESULT,
	fn Unmap(&mut self) -> HRESULT
}}
com_interface!{ IDXGISurface1(IDXGISurface1Vtbl): IDXGISurface(IDXGISurfaceVtbl) {
	fn GetDC(&mut self, discars: BOOL, dirtyrect: *mut RECT) -> HRESULT,
	fn ReleaseDC(&mut self, dirtyrect: *mut RECT) -> HRESULT
}}
com_interface!{ IDXGISurface2(IDXGISurface2Vtbl): IDXGISurface1(IDXGISurface1Vtbl) {
	fn GetResource(&mut self, riid: REFIID, parent_resource: *mut *mut c_void, subresource_index: *mut UINT) -> HRESULT
}}
com_interface!{ IDXGISwapChain(IDXGISwapChainVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
	fn Present(&mut self, synt_interval: UINT, flags: UINT) -> HRESULT,
	fn GetBuffer(&mut self, buffer: UINT, riid: REFIID, surface: *mut *mut c_void) -> HRESULT,
	fn SetFullscreenState(&mut self, fullscreen: BOOL, target: *mut IDXGIOutput) -> HRESULT,
	fn GetFullscreenState(&mut self, fullscreen: *mut BOOL, target: *mut *mut IDXGIOutput) -> HRESULT,
	fn GetDesc(&mut self, desc: *mut DXGI_SWAP_CHAIN_DESC) ->HRESULT,
	fn ResizeBuffers(&mut self, bufcount: UINT, width: UINT, height: UINT, new_format: DXGI_FORMAT, swapchain_flags: UINT) -> HRESULT,
	fn ResizeTarget(&mut self, new_target_params: *const DXGI_MODE_DESC) -> HRESULT,
	fn GetContainingOutput(&mut self, output: *mut *mut IDXGIOutput) -> HRESULT,
	fn GetFrameStatistics(&mut self, stats: *mut DXGI_FRAME_STATISTICS) -> HRESULT,
	fn GetLastPresentCount(&mut self, last_present_count: *mut UINT) -> HRESULT
}}
com_interface!{ IDXGISwapChain1(IDXGISwapChain1Vtbl): IDXGISwapChain(IDXGISwapChainVtbl) {
	fn GetDesc1(&mut self, desc: *mut DXGI_SWAP_CHAIN_DESC1) -> HRESULT,
	fn GetFullscreenDesc(&mut self, desc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> HRESULT,
	fn GetHwnd(&mut self, hwnd: *mut HWND) -> HRESULT,
	fn GetCoreWindow(&mut self, refiid: REFIID, unk: *mut *mut c_void) -> HRESULT,
	fn Present1(&mut self, sync_interval: UINT, present_flags: UINT, present_parameters: *const DXGI_PRESENT_PARAMETERS) -> HRESULT,
	fn IsTemporaryMonoSupported(&mut self) -> BOOL,
	fn GetRestrictToOutput(&mut self, restrict_to_output: *mut *mut IDXGIOutput) -> HRESULT,
	fn SetBackgroundColor(&mut self, color: *const DXGI_RGBA) -> HRESULT,
	fn GetBackgroundColor(&mut self, color: *mut DXGI_RGBA) -> HRESULT,
	fn SetRotation(&mut self, rotation: DXGI_MODE_ROTATION) -> HRESULT,
	fn GetRotation(&mut self, rotation: *mut DXGI_MODE_ROTATION) -> HRESULT
}}
com_interface!{ IDXGISwapChain2(IDXGISwapChain2Vtbl): IDXGISwapChain1(IDXGISwapChain1Vtbl) {
	fn SetSourceSize(&mut self, width: UINT, height: UINT) -> HRESULT,
	fn GetSourceSize(&mut self, width: *mut UINT, height: *mut UINT) -> HRESULT,
	fn SetMaximumFrameLatency(&mut self, max_latency: UINT) -> HRESULT,
	fn GetMaximumFrameLatency(&mut self, max_latency: *mut UINT) -> HRESULT,
	fn GetFrameLatencyWaitableObject(&mut self) -> HANDLE,
	fn SetMatrixTransform(&mut self, matrix: *const DXGI_MATRIX_3X2_F) -> HRESULT,
	fn GetMatrixTransform(&mut self, matrix: *mut DXGI_MATRIX_3X2_F) -> HRESULT
}}
com_interface!{ IDXGIFactory(IDXGIFactoryVtbl): IDXGIObject(IDXGIObjectVtbl) {
	fn EnumAdapters(&mut self, adapter_i: UINT, adapter: *mut *mut IDXGIAdapter) -> HRESULT,
	fn MakeWindowAssociation(&mut self, window_handle: HWND, flags: UINT) -> HRESULT,
	fn GetWindowAssociation(&mut self, window_handle: *mut HWND) -> HRESULT,
	fn CreateSwapChain(&mut self, device: *mut IUnknown, desc: *mut DXGI_SWAP_CHAIN_DESC, swapchain: *mut *mut IDXGISwapChain) -> HRESULT,
	fn CreateSoftwareAdapter(&mut self, module: HMODULE, adapter: *mut *mut IDXGIAdapter) -> HRESULT
}}
com_interface!{ IDXGIFactory1(IDXGIFactory1Vtbl): IDXGIFactory(IDXGIFactoryVtbl) {
	fn EnumAdapters1(&mut self, adapter_i: UINT, adapter: *mut *mut IDXGIAdapter1) -> HRESULT,
	fn IsCurrent(&mut self) -> BOOL
}}
com_interface!{ IDXGIFactory2(IDXGIFactory2Vtbl): IDXGIFactory1(IDXGIFactory1Vtbl) {
	fn IsWindowedStereoEnabled(&mut self) -> BOOL,
	fn CreateSwapChainForHwnd(&mut self, device: *mut IUnknown, hwnd: HWND, desc: *const DXGI_SWAP_CHAIN_DESC1, fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, restrict_to_output: *mut IDXGIOutput, swapchain: *mut *mut IDXGISwapChain1) -> HRESULT,
	fn CreateSwapChainForCoreWindow(&mut self, device: *mut IUnknown, window: *mut IUnknown, desc: *const DXGI_SWAP_CHAIN_DESC1, restrict_to_output: *mut DXGI_SWAP_CHAIN_DESC1) -> HRESULT,
	fn GetSharedResourceAdapterLuid(&mut self, resource: HANDLE, luid: *mut LUID) -> HRESULT,
	fn RegisterStereoStatusWindow(&mut self, window_handle: HWND, msg: UINT, cookie: *mut DWORD) -> HRESULT,
	fn RegisterStereoStatusEvent(&mut self, event_handle: HANDLE, cookie: *mut DWORD) -> HRESULT,
	fn UnregisterStereoStatus(&mut self, cookit: DWORD) -> (),
	fn RegisterOcclusionStatusWindow(&mut self, window_handle: HWND, msg: UINT, cookie: *mut DWORD) -> HRESULT,
	fn RegisterOcclusionStatusEvent(&mut self, event_handle: HANDLE, cookie: *mut DWORD) -> HRESULT,
	fn UnregisterOcclusionStatus(&mut self, cookie: DWORD) -> (),
	fn CreateSwapChainForComposition(&mut self, device: *mut IUnknown, desc: *const DXGI_SWAP_CHAIN_DESC1, restrict_to_output: *mut IDXGIOutput, swapchain: *mut *mut IDXGISwapChain1) -> HRESULT
}}
com_interface!{ IDXGIFactory3(IDXGIFactory3Vtbl): IDXGIFactory2(IDXGIFactory2Vtbl) {
	fn GetCreationFlags(&mut self) -> UINT
}}
com_interface!{ IDXGIFactoryMedia(IDXGIFactoryMediaVtbl): IDXGIObject(IDXGIObjectVtbl) {
	fn CreateSwapChainForCompositionSurfaceHandle(&mut self, device: *mut IUnknown, surface_handle: HANDLE, desc: *const DXGI_SWAP_CHAIN_DESC1, restrict_to_output: *mut IDXGIOutput, swapchain: *mut *mut IDXGISwapChain1) -> HRESULT,
	fn CreateDecodeSwapChainForCompositionSurfaceHandle(&mut self, device: *mut IUnknown, surface_handle: HANDLE, desc: *mut DXGI_DECODE_SWAP_CHAIN_DESC, yuv_decode_bufs: *mut IDXGIResource, restrict_to_output: *mut IDXGIOutput, swapchain: *mut *mut IDXGIDecodeSwapChain) -> HRESULT
}}
com_interface!{ IDXGIOutput(IDXGIOutputVtbl): IDXGIObject(IDXGIObjectVtbl) {
	fn GetDesc(&mut self, desc: *mut DXGI_OUTPUT_DESC) -> HRESULT,
	fn GetDisplayModeList(&mut self, enum_format: DXGI_FORMAT, flags: UINT, num_modes: *mut UINT, desc: *mut DXGI_MODE_DESC) -> HRESULT,
	fn FingClosestMatchingMode(&mut self, mode_to_match: *const DXGI_MODE_DESC, closest_match: *mut DXGI_MODE_DESC, concerned_device: IUnknown) -> HRESULT,
	fn WaitForVBlank(&mut self) -> HRESULT,
	fn TakeOwnerShip(&mut self, device: *mut IUnknown, exclusive: BOOL) -> HRESULT,
	fn ReleaseOwnership(&mut self) -> (),
	fn GetGammaControlCapabilities(&mut self, gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES) -> HRESULT,
	fn SetGammaControl(&mut self, array: *const DXGI_GAMMA_CONTROL) -> HRESULT,
	fn GetGammaControl(&mut self, arrau: *mut DXGI_GAMMA_CONTROL) -> HRESULT,
	fn SetDisplaySurface(&mut self, scanout_surface: *mut IDXGISurface) -> HRESULT,
	fn GetDispleySurfaceData(&mut self, destination: *mut IDXGISurface) -> HRESULT,
	fn GetFrameStatistics(&mut self, stats: *mut DXGI_FRAME_STATISTICS) -> HRESULT
}}
com_interface!{ IDXGIOutput1(IDXGIOutput1Vtbl): IDXGIOutput(IDXGIOutputVtbl) {
	fn GetDisplayModeList1(&mut self, enum_format: DXGI_FORMAT, flags: UINT, num_modes: *mut UINT, desc: *mut DXGI_MODE_DESC1) -> HRESULT,
	fn FindClosestMatchingMode1(&mut self, mode_to_match: *const DXGI_MODE_DESC1, closest_match: *mut DXGI_MODE_DESC1, concerned_device: *mut IUnknown) -> HRESULT,
	fn GetDisplaySurfaceData1(&mut self, destination: *mut IDXGIResource) -> HRESULT,
	fn DuplicateOutput(&mut self, device: *mut IUnknown, output_duplication: *mut *mut IDXGIOutputDuplication) -> HRESULT
}}
com_interface!{ IDXGIOutput2(IDXGIOutput2Vtbl): IDXGIOutput1(IDXGIOutput1Vtbl) {
	fn SupportsOverlay(&mut self) -> BOOL
}}
com_interface!{ IDXGIOutputDuplication(IDXGIOutputDuplicationVtbl): IDXGIObject(IDXGIObjectVtbl) {
	fn GetDesc(&mut self, desc: *mut DXGI_OUTDUPL_DESC) -> (),
	fn AcquireNextFrame(&mut self, timeout_ms: UINT, frame_info: *mut DXGI_OUTDUPL_FRAME_INFO, desktop_resource: *mut *mut IDXGIResource) -> HRESULT,
	fn GetFrameDirtyRects(&mut self, dirty_rects_bufsize: UINT, dirtyrects_buf: *mut RECT, dirtyrects_bufsize_required: *mut UINT) -> HRESULT,
	fn GetFrameMoveRects(&mut self, moverects_bufsize: UINT, moverect_buf: *mut DXGI_OUTDUPL_MOVE_RECT, moverects_bufsize_required: UINT) -> HRESULT,
	fn GetFramePointerShape(&mut self, pointershape_bufsize: UINT, pointershape_buf: *mut c_void, pointershape_bufsize_required: *mut UINT, pointershape_info: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> HRESULT,
	fn MapDesktopSurface(&mut self, locked_rect: *mut DXGI_MAPPED_RECT) -> HRESULT,
	fn UmMapDesktopSurface(&mut self) -> HRESULT,
	fn ReleaseFrame(&mut self) -> HRESULT
}}
com_interface!{ IDXGIDebug(IDXGIDebugVtbl): IUnknown(IUnknownVtbl) {
	fn ReportLiveObjects(&mut self, apiid: GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> HRESULT
}}
com_interface!{ IDXGIDebug1(IDXGIDebug1Vtbl): IDXGIDebug(IDXGIDebugVtbl) {
	fn EnableLeakTrackingForThread(&mut self) -> (),
	fn DisableLeakTrackingForThread(&mut self) -> (),
	fn IsLeakTrckingEnabledForThread(&mut self) -> ()
}}
com_interface!{ IDXGIDecodeSwapChain(IDXGIDecodeSwapChainVtbl): IUnknown(IUnknownVtbl) {
	fn GetColorSpace(&mut self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
	fn GetDestSize(&mut self, width: *mut UINT, height: *mut UINT) -> HRESULT,
	fn GetSourceRect(&mut self, rect: *mut RECT) -> HRESULT,
	fn GetTargetRect(&mut self, rect: *mut RECT) -> HRESULT,
	fn PresentBuffer(&mut self, buffet_to_present: UINT, SyncInterval: UINT, flags: UINT) -> HRESULT,
	fn SetColorSpace(&mut self, color_space: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> HRESULT,
	fn SetDestSize(&mut self, width: UINT, height: UINT) -> HRESULT,
	fn SetSourceRect(&mut self, rect: *const RECT) -> HRESULT,
	fn SetTargetRect(&mut self, rect: *const RECT) -> HRESULT
}}
com_interface!{ IDXGIDisplayControl(IDXGIDisplayControlVtbl): IUnknown(IUnknownVtbl) {
	fn IsStereoEnabled(&mut self) -> BOOL,
	fn SetStereoEnabled(&mut self, enabled: BOOL) -> ()
}}
com_interface!{ IDXGIInfoQueue(IDXGIInfoQueueVtbl): IUnknown(IUnknownVtbl) {
	fn SetMessageCountLimit(&mut self, producer: DXGI_DEBUG_ID, msg_count_limit: UINT64) -> HRESULT,
	fn ClearStoredMessages(&mut self, producer: DXGI_DEBUG_ID) -> (),
	fn GetMessage(&mut self, producer: DXGI_DEBUG_ID, message_i: UINT64, message: *mut DXGI_INFO_QUEUE_MESSAGE, msg_byte_len: *mut SIZE_T) -> HRESULT,
	fn GetNumStoredMessagesAllowedByRetrievalFilters(&mut self, producer: DXGI_DEBUG_ID) -> UINT64,
	fn GetNumStoredMessages(&mut self, producer: DXGI_DEBUG_ID) -> UINT64,
	fn GetNumMessagesDiscardedByMessageCountLimit(&mut self, producer: DXGI_DEBUG_ID) -> UINT64,
	fn GetMessageCountLimit(&mut self, producer: DXGI_DEBUG_ID) -> UINT64,
	fn GetNumMessagesAllowedByStorageFilter(&mut self, producer: DXGI_DEBUG_ID) -> UINT64,
	fn GetNumMessagesDeniedByStorageFilter(&mut self, producer: DXGI_DEBUG_ID) -> UINT64,
	fn AddStorageFilterEntries(&mut self, producer: DXGI_DEBUG_ID, filter: *mut DXGI_INFO_QUEUE_FILTER) -> HRESULT,
	fn GetStorageFilter(&mut self, producer: DXGI_DEBUG_ID, filter: *mut DXGI_INFO_QUEUE_FILTER, filter_byte_len: *mut SIZE_T) -> HRESULT,
	fn ClearStorageFilter(&mut self, producer: DXGI_DEBUG_ID) -> (),
	fn PushEmptyStorageFilter(&mut self, producer: DXGI_DEBUG_ID) -> HRESULT,
	fn PushDenyAllStorageFilter(&mut self, producer: DXGI_DEBUG_ID) -> HRESULT,
	fn PushCopyOfStorageFilter(&mut self, producer: DXGI_DEBUG_ID) -> HRESULT,
	fn PushStorageFilter(&mut self, producer: DXGI_DEBUG_ID, filter: *mut DXGI_INFO_QUEUE_FILTER) -> HRESULT,
	fn PopStorageFilter(&mut self, producer: DXGI_DEBUG_ID) -> (),
	fn GetStorageFilterStackSize(&mut self, producer: DXGI_DEBUG_ID) -> UINT,
	fn AddRetrievalFilterEntries(&mut self, producer: DXGI_DEBUG_ID, filter: *mut DXGI_INFO_QUEUE_FILTER) -> HRESULT,
	fn GetRetrievalFilter(&mut self, producer: DXGI_DEBUG_ID, filter: *mut DXGI_INFO_QUEUE_FILTER, filter_byte_len: *mut SIZE_T) -> HRESULT,
	fn ClearRetrievalFilter(&mut self, producer: DXGI_DEBUG_ID) -> (),
	fn PushEmptyRetrievalFilter(&mut self, producer: DXGI_DEBUG_ID) -> HRESULT,
	fn PushDenyAllRetrievalFilter(&mut self, producer: DXGI_DEBUG_ID) -> HRESULT,
	fn PushCopyOfRetrievalFilter(&mut self, producer: DXGI_DEBUG_ID) -> HRESULT,
	fn PushRetrievalFilter(&mut self, producer: DXGI_DEBUG_ID, filter: *mut DXGI_INFO_QUEUE_FILTER) -> HRESULT,
	fn PopRetrievalFilter(&mut self, producer: DXGI_DEBUG_ID) -> (),
	fn GetRetrievalFilterStackSize(&mut self, producer: DXGI_DEBUG_ID) -> UINT,
	fn AddMessage(&mut self, producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: DXGI_INFO_QUEUE_MESSAGE_ID, desc: LPCSTR) -> HRESULT,
	fn AddApplicationMessage(&mut self, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, desc: LPCSTR) -> HRESULT,
	fn SetBreakOnCategory(&mut self, producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, enable: BOOL) -> HRESULT,
	fn SetBreakOnSeverity(&mut self, producer: DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, enable: BOOL) -> HRESULT,
	fn SetBreakOnID(&mut self, producer: DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID, enable: BOOL) -> HRESULT,
	fn GetBreakOnCategory(&mut self, producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> BOOL,
	fn GetBreakOnSeverity(&mut self, producer: DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> BOOL,
	fn GetBreakOnID(&mut self, producer: DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID) -> BOOL,
	fn SetMuteDebugOutput(&mut self, producer: DXGI_DEBUG_ID, mute: BOOL) -> (),
	fn GetMuteDebugOutput(&mut self, producer: DXGI_DEBUG_ID) -> BOOL
}}
com_interface!{ IDXGISwapChainMedia(IDXGISwapChainMediaVtbl): IUnknown(IUnknownVtbl) {
	fn GetFrameStatisticsMedia(&mut self, stats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> HRESULT,
	fn SetPresentDuration(&mut self, duration: UINT) -> HRESULT,
	fn CheckPresentDurationSupport(&mut self, desired_present_duration: UINT, closest_smaller_present_duration: *mut UINT, closest_larger_present_duration: *mut UINT) -> HRESULT
}}

// TODO: Replace function with associated const when no longer behind feature gate
pub trait QueryIID {
	fn iid() -> GUID;
}
impl QueryIID for IDXGIAdapter { fn iid() -> GUID { IID_IDXGIAdapter } }
impl QueryIID for IDXGIAdapter1 { fn iid() -> GUID { IID_IDXGIAdapter1 } }
impl QueryIID for IDXGIAdapter2 { fn iid() -> GUID { IID_IDXGIAdapter2 } }
impl QueryIID for IDXGIDebug { fn iid() -> GUID { IID_IDXGIDebug } }
impl QueryIID for IDXGIDebug1 { fn iid() -> GUID { IID_IDXGIDebug1 } }
impl QueryIID for IDXGIDecodeSwapChain { fn iid() -> GUID { IID_IDXGIDecodeSwapChain } }
impl QueryIID for IDXGIDevice { fn iid() -> GUID { IID_IDXGIDevice } }
impl QueryIID for IDXGIDevice1 { fn iid() -> GUID { IID_IDXGIDevice1 } }
impl QueryIID for IDXGIDevice2 { fn iid() -> GUID { IID_IDXGIDevice2 } }
impl QueryIID for IDXGIDevice3 { fn iid() -> GUID { IID_IDXGIDevice3 } }
impl QueryIID for IDXGIDeviceSubObject { fn iid() -> GUID { IID_IDXGIDeviceSubObject } }
impl QueryIID for IDXGIDisplayControl { fn iid() -> GUID { IID_IDXGIDisplayControl } }
impl QueryIID for IDXGIFactory { fn iid() -> GUID { IID_IDXGIFactory } }
impl QueryIID for IDXGIFactory1 { fn iid() -> GUID { IID_IDXGIFactory1 } }
impl QueryIID for IDXGIFactory2 { fn iid() -> GUID { IID_IDXGIFactory2 } }
impl QueryIID for IDXGIFactory3 { fn iid() -> GUID { IID_IDXGIFactory3 } }
impl QueryIID for IDXGIFactoryMedia { fn iid() -> GUID { IID_IDXGIFactoryMedia } }
impl QueryIID for IDXGIInfoQueue { fn iid() -> GUID { IID_IDXGIInfoQueue } }
impl QueryIID for IDXGIKeyedMutex { fn iid() -> GUID { IID_IDXGIKeyedMutex } }
impl QueryIID for IDXGIObject { fn iid() -> GUID { IID_IDXGIObject } }
impl QueryIID for IDXGIOutput { fn iid() -> GUID { IID_IDXGIOutput } }
impl QueryIID for IDXGIOutput1 { fn iid() -> GUID { IID_IDXGIOutput1 } }
impl QueryIID for IDXGIOutput2 { fn iid() -> GUID { IID_IDXGIOutput2 } }
impl QueryIID for IDXGIOutputDuplication { fn iid() -> GUID { IID_IDXGIOutputDuplication } }
impl QueryIID for IDXGIResource { fn iid() -> GUID { IID_IDXGIResource } }
impl QueryIID for IDXGIResource1 { fn iid() -> GUID { IID_IDXGIResource1 } }
impl QueryIID for IDXGISurface { fn iid() -> GUID { IID_IDXGISurface } }
impl QueryIID for IDXGISurface1 { fn iid() -> GUID { IID_IDXGISurface1 } }
impl QueryIID for IDXGISurface2 { fn iid() -> GUID { IID_IDXGISurface2 } }
impl QueryIID for IDXGISwapChain { fn iid() -> GUID { IID_IDXGISwapChain } }
impl QueryIID for IDXGISwapChain1 { fn iid() -> GUID { IID_IDXGISwapChain1 } }
impl QueryIID for IDXGISwapChain2 { fn iid() -> GUID { IID_IDXGISwapChain2 } }
impl QueryIID for IDXGISwapChainMedia { fn iid() -> GUID { IID_IDXGISwapChainMedia } }
