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
//! [DXGI Interfaces, MSDN](https://msdn.microsoft.com/en-us/library/windows/desktop/ff471322(v=vs.85).aspx)

use libc::{ c_void };
use winapi::{ HRESULT, ULONG, REFGUID,
	UINT, IUnknown, REFIID,
	LARGE_INTEGER, GUID, HWND,
	HMODULE, BOOL, INT,
	HANDLE, RECT, LUID,
	DWORD };

use dxgi_structures::*;
use dxgi_enumerations::*;
use dxgi_constants::*;

#[repr(C)] pub struct IDXGIAdapter {
	lpVtbl: *mut IDXGIAdapterVtbl
}
#[repr(C)] pub struct IDXGIAdapter1 {
	lpVtbl: *mut IDXGIAdapter1Vtbl
}
#[repr(C)] pub struct IDXGIAdapter2 {
	lpVtbl: *mut IDXGIAdapter2Vtbl
}
#[repr(C)] pub struct IDXGIDebug {
	lpVtbl: *mut IDXGIDebugVtbl
}
#[repr(C)] pub struct IDXGIDebug1 {
	lpVtbl: *mut IDXGIDebug1Vtbl
}
#[repr(C)] pub struct IDXGIDecodeSwapChain {
	lpVtbl: *mut IDXGIDecodeSwapChainVtbl
}
#[repr(C)] pub struct IDXGIDevice {
	lpVtbl: *mut IDXGIDeviceVtbl
}
#[repr(C)] pub struct IDXGIDevice1 {
	lpVtbl: *mut IDXGIDevice1Vtbl
}
#[repr(C)] pub struct IDXGIDevice2 {
	lpVtbl: *mut IDXGIDevice2Vtbl
}
#[repr(C)] pub struct IDXGIDevice3 {
	lpVtbl: *mut IDXGIDevice3Vtbl
}
#[repr(C)] pub struct IDXGIDeviceSubObject {
	lpVtbl: *mut IDXGIDeviceSubObjectVtbl
}
#[repr(C)] pub struct IDXGIDisplayControl {
	lpVtbl: *mut IDXGIDisplayControlVtbl
}
#[repr(C)] pub struct IDXGIFactory {
	lpVtbl: *mut IDXGIFactoryVtbl
}
#[repr(C)] pub struct IDXGIFactory1 {
	lpVtbl: *mut IDXGIFactory1Vtbl
}
#[repr(C)] pub struct IDXGIFactory2 {
	lpVtbl: *mut IDXGIFactory2Vtbl
}
#[repr(C)] pub struct IDXGIFactory3 {
	lpVtbl: *mut IDXGIFactory3Vtbl
}
#[repr(C)] pub struct IDXGIFactoryMedia {
	lpVtbl: *mut IDXGIFactoryMediaVtbl
}
#[repr(C)] pub struct IDXGIInfoQueue {
	lpVtbl: *mut IDXGIInfoQueueVtbl
}
#[repr(C)] pub struct IDXGIKeyedMutex {
	lpVtbl: *mut IDXGIKeyedMutexVtbl
}
#[repr(C)] pub struct IDXGIObject {
	lpVtbl: *mut IDXGIObjectVtbl
}
#[repr(C)] pub struct IDXGIOutput {
	lpVtbl: *mut IDXGIOutputVtbl
}
#[repr(C)] pub struct IDXGIOutput1 {
	lpVtbl: *mut IDXGIOutput1Vtbl
}
#[repr(C)] pub struct IDXGIOutput2 {
	lpVtbl: *mut IDXGIOutput2Vtbl
}
#[repr(C)] pub struct IDXGIOutputDuplication {
	lpVtbl: *mut IDXGIOutputDuplicationVtbl
}
#[repr(C)] pub struct IDXGIResource {
	lpVtbl: *mut IDXGIResourceVtbl
}
#[repr(C)] pub struct IDXGIResource1 {
	lpVtbl: *mut IDXGIResource1Vtbl
}
#[repr(C)] pub struct IDXGISurface {
	lpVtbl: *mut IDXGISurfaceVtbl
}
#[repr(C)] pub struct IDXGISurface1 {
	lpVtbl: *mut IDXGISurface1Vtbl
}
#[repr(C)] pub struct IDXGISurface2 {
	lpVtbl: *mut IDXGISurface2Vtbl
}
#[repr(C)] pub struct IDXGISwapChain {
	lpVtbl: *mut IDXGISwapChainVtbl
}
#[repr(C)] pub struct IDXGISwapChain1 {
	lpVtbl: *mut IDXGISwapChain1Vtbl
}
#[repr(C)] pub struct IDXGISwapChain2 {
	lpVtbl: *mut IDXGISwapChain2Vtbl
}
#[repr(C)] pub struct IDXGISwapChainMedia {
	lpVtbl: *mut IDXGISwapChainMediaVtbl
}

#[repr(C)] c_vtable!(
// We only need the methods for inheritance, the actual IUnknown is imported from winapi
IUnknownVtbl of () {
	fn QueryInterface(riid: REFIID, object: *mut *mut c_void) -> HRESULT,
	fn AddRef() -> ULONG,
	fn Release() -> ULONG,
} with heirs [
	IDXGIObjectVtbl of IDXGIObject {
		fn SetPrivateData(name: REFGUID, data_size: UINT, data: *const c_void) -> HRESULT,
		fn SetPrivateDataInterface(name: REFGUID, unknown: *const IUnknown) -> HRESULT,
		fn GetPrivateData(name: REFGUID, data_size: UINT, data: *mut c_void) -> HRESULT,
		fn GetParent(riid: REFIID, parent: *mut *mut c_void) -> HRESULT,
	} with heirs [
		IDXGIAdapterVtbl of IDXGIAdapter {
			fn EnumOutputs(output_i: UINT, output:*mut *mut IDXGIOutput) -> HRESULT,
			fn GetDesc(desc: *mut *mut DXGI_ADAPTER_DESC) -> HRESULT,
			fn CheckInterfaceSupport(interface_name: REFGUID, umd_version: LARGE_INTEGER) -> HRESULT,
		} with heirs [
			IDXGIAdapter1Vtbl of IDXGIAdapter1 {
				fn GetDesc1(desc: *mut DXGI_ADAPTER_DESC1) -> HRESULT,
			}
			IDXGIAdapter2Vtbl of IDXGIAdapter2 {
				fn GetDesc2(desc: *mut DXGI_ADAPTER_DESC2) -> HRESULT,
			}
		]
		IDXGIDeviceVtbl of IDXGIDevice {
			fn GetAdapter(adapter: *mut *mut IDXGIAdapter) -> HRESULT,
			fn CreateSurface(desc: *const DXGI_SURFACE_DESC, num_surfaces: UINT, usage: DXGI_USAGE,
				shared_resource: *const DXGI_SHARED_RESOURCE, surface: *mut *mut IDXGISurface)
				-> HRESULT,
			fn QueryResourceResidency(resources: *const IUnknown,
				residency_status: *mut DXGI_RESIDENCY, num_resources: UINT) -> HRESULT,
			fn SetGPUThreadPriority(priority: INT) -> HRESULT,
			fn GetGPUThreadPriority(priority: *mut INT) -> HRESULT,
		} with heirs [
			IDXGIDevice1Vtbl of IDXGIDevice1 {
				fn GetMaximumFrameLatency(max_latency: *mut UINT) -> HRESULT,
				fn SetMaximumFrameLatency(max_latency: UINT) -> HRESULT,
			} with heirs [
				IDXGIDevice2Vtbl of IDXGIDevice2 {
					fn OfferResources(num_resources: UINT, resources: *const *mut IDXGIResource,
						priority: DXGI_OFFER_RESOURCE_PRIORITY) -> HRESULT,
					fn ReclaimResources(num_resources: UINT, resources: *const *mut IDXGIResource,
						discarded: *mut BOOL) -> HRESULT,
					fn EnqueueSetEvent(event: HANDLE) -> HRESULT,
				} with heirs [
					IDXGIDevice3Vtbl of IDXGIDevice3 {
						fn Trim() -> (),
					}
				]
			]
		]
		IDXGIDeviceSubObjectVtbl of IDXGIDeviceSubObject {
			fn GetDevice(riid: REFIID, device: *mut *mut c_void) -> HRESULT,
		}
		IDXGIFactoryVtbl of IDXGIFactory {
			fn EnumAdapters(adapter_i: UINT, adapter: *mut *mut IDXGIAdapter) -> HRESULT,
			fn MakeWindowAssociation(window_handle: HWND, flags: UINT) -> HRESULT,
			fn GetWindowAssociation(window_handle: *mut HWND) -> HRESULT,
			fn CreateSwapChain(device: *mut IUnknown, desc: *mut DXGI_SWAP_CHAIN_DESC,
				swapchain: *mut *mut IDXGISwapChain) -> HRESULT,
			fn CreateSoftwareAdapter(module: HMODULE, adapter: *mut *mut IDXGIAdapter) -> HRESULT,
		} with heirs [
			IDXGIFactory1Vtbl of IDXGIFactory1 {
				fn EnumAdapters1(adapter_i: UINT, adapter: *mut *mut IDXGIAdapter1) -> HRESULT,
				fn IsCurrent() -> BOOL,
			} with heirs [
				IDXGIFactory2Vtbl of IDXGIFactory2 {
					fn IsWindowedStereoEnabled() -> BOOL,
					fn CreateSwapChainForHwnd(device: *mut IUnknown, hwnd: HWND,
						desc: *const DXGI_SWAP_CHAIN_DESC1,
						fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
						restrict_to_output: *mut IDXGIOutput, swapchain: *mut *mut IDXGISwapChain1)
						-> HRESULT,
					fn CreateSwapChainForCoreWindow(device: *mut IUnknown, window: *mut IUnknown,
						desc: *const DXGI_SWAP_CHAIN_DESC1,
						restrict_to_output: *mut DXGI_SWAP_CHAIN_DESC1) -> HRESULT,
					fn GetSharedResourceAdapterLuid(resource: HANDLE, luid: *mut LUID) -> HRESULT,
					fn RegisterStereoStatusWindow(window_handle: HWND, msg: UINT,
						cookie: *mut DWORD) -> HRESULT,
					fn RegisterStereoStatusEvent(event_handle: HANDLE, cookie: *mut DWORD)
						-> HRESULT,
					fn UnregisterStereoStatus(cookit: DWORD) -> (),
					fn RegisterOcclusionStatusWindow(window_handle: HWND, msg: UINT,
						cookie: *mut DWORD) -> HRESULT,
					fn RegisterOcclusionStatusEvent(event_handle: HANDLE, cookie: *mut DWORD)
						-> HRESULT,
					fn UnregisterOcclusionStatus(cookie: DWORD) -> (),
					fn CreateSwapChainForComposition(device: *mut IUnknown,
						desc: *const DXGI_SWAP_CHAIN_DESC1, restrict_to_output: *mut IDXGIOutput,
						swapchain: *mut *mut IDXGISwapChain1) -> HRESULT,
				} with heirs [
					IDXGIFactory3Vtbl of IDXGIFactory3 {
						fn GetCreationFlags() -> UINT,
					}
				]
			]
		]
		IDXGIOutputVtbl of IDXGIOutput {
			fn GetDesc(desc: *mut DXGI_OUTPUT_DESC) -> HRESULT,
			fn GetDisplayModeList(enum_format: DXGI_FORMAT, flags: UINT, num_modes: *mut UINT,
				desc: *mut DXGI_MODE_DESC) -> HRESULT,
			fn FingClosestMatchingMode(mode_to_match: *const DXGI_MODE_DESC,
				closest_match: *mut DXGI_MODE_DESC, concerned_device: IUnknown) -> HRESULT,
			fn WaitForVBlank() -> HRESULT,
			fn TakeOwnerShip(device: *mut IUnknown, exclusive: BOOL) -> HRESULT,
			fn ReleaseOwnership() -> (),
			fn GetGammaControlCapabilities(gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES) -> HRESULT,
			fn SetGammaControl(array: *const DXGI_GAMMA_CONTROL) -> HRESULT,
			fn GetGammaControl(arrau: *mut DXGI_GAMMA_CONTROL) -> HRESULT,
			fn SetDisplaySurface(scanout_surface: *mut IDXGISurface) -> HRESULT,
			fn GetDispleySurfaceData(destination: *mut IDXGISurface) -> HRESULT,
			fn GetFrameStatistics(stats: *mut DXGI_FRAME_STATISTICS) -> HRESULT,
		}
	]
	IDXGIDebugVtbl of IDXGIDebug {
		fn ReportLiveObjects(apiid: GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> HRESULT,
	} with heirs [
		IDXGIDebug1Vtbl of IDXGIDebug1 {
			fn EnableLeakTrackingForThread() -> (),
			fn DisableLeakTrackingForThread() -> (),
			fn IsLeakTrckingEnabledForThread() -> (),
		}
	]
	IDXGIDecodeSwapChainVtbl of IDXGIDecodeSwapChain {
		fn GetColorSpace() -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
		fn GetDestSize(width: *mut UINT, height: *mut UINT) -> HRESULT,
		fn GetSourceRect(rect: *mut RECT) -> HRESULT,
		fn GetTargetRect(rect: *mut RECT) -> HRESULT,
		fn PresentBuffer(buffet_to_present: UINT, SyncInterval: UINT, flags: UINT) -> HRESULT,
		fn SetColorSpace(color_space: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> HRESULT,
		fn SetDestSize(width: UINT, height: UINT) -> HRESULT,
		fn SetSourceRect(rect: *const RECT) -> HRESULT,
		fn SetTargetRect(rect: *const RECT) -> HRESULT,
	}
	IDXGIDisplayControlVtbl of IDXGIDisplayControl {
		fn IsStereoEnabled() -> BOOL,
		fn SetStereoEnabled(enabled: BOOL) -> (),
	}
]);