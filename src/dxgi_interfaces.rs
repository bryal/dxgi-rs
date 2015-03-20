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

#[repr(C)] pub struct IDXGIAdapter {
	vtable: *mut IDXGIAdapterVtbl
}
#[repr(C)] pub struct IDXGIAdapter1 {
	vtable: *mut IDXGIAdapter1Vtbl
}
#[repr(C)] pub struct IDXGIAdapter2 {
	vtable: *mut IDXGIAdapter2Vtbl
}
#[repr(C)] pub struct IDXGIDebug {
	vtable: *mut IDXGIDebugVtbl
}
#[repr(C)] pub struct IDXGIDebug1 {
	vtable: *mut IDXGIDebug1Vtbl
}
#[repr(C)] pub struct IDXGIDecodeSwapChain {
	vtable: *mut IDXGIDecodeSwapChainVtbl
}
#[repr(C)] pub struct IDXGIDevice {
	vtable: *mut IDXGIDeviceVtbl
}
#[repr(C)] pub struct IDXGIDevice1 {
	vtable: *mut IDXGIDevice1Vtbl
}
#[repr(C)] pub struct IDXGIDevice2 {
	vtable: *mut IDXGIDevice2Vtbl
}
#[repr(C)] pub struct IDXGIDevice3 {
	vtable: *mut IDXGIDevice3Vtbl
}
#[repr(C)] pub struct IDXGIDeviceSubObject {
	vtable: *mut IDXGIDeviceSubObjectVtbl
}
#[repr(C)] pub struct IDXGIDisplayControl {
	vtable: *mut IDXGIDisplayControlVtbl
}
#[repr(C)] pub struct IDXGIFactory {
	vtable: *mut IDXGIFactoryVtbl
}
#[repr(C)] pub struct IDXGIFactory1 {
	vtable: *mut IDXGIFactory1Vtbl
}
#[repr(C)] pub struct IDXGIFactory2 {
	vtable: *mut IDXGIFactory2Vtbl
}
#[repr(C)] pub struct IDXGIFactory3 {
	vtable: *mut IDXGIFactory3Vtbl
}
#[repr(C)] pub struct IDXGIFactoryMedia {
	vtable: *mut IDXGIFactoryMediaVtbl
}
#[repr(C)] pub struct IDXGIInfoQueue {
	vtable: *mut IDXGIInfoQueueVtbl
}
#[repr(C)] pub struct IDXGIKeyedMutex {
	vtable: *mut IDXGIKeyedMutexVtbl
}
#[repr(C)] pub struct IDXGIObject {
	vtable: *mut IDXGIObjectVtbl
}
#[repr(C)] pub struct IDXGIOutput {
	vtable: *mut IDXGIOutputVtbl
}
#[repr(C)] pub struct IDXGIOutput1 {
	vtable: *mut IDXGIOutput1Vtbl
}
#[repr(C)] pub struct IDXGIOutput2 {
	vtable: *mut IDXGIOutput2Vtbl
}
#[repr(C)] pub struct IDXGIOutputDuplication {
	vtable: *mut IDXGIOutputDuplicationVtbl
}
#[repr(C)] pub struct IDXGIResource {
	vtable: *mut IDXGIResourceVtbl
}
#[repr(C)] pub struct IDXGIResource1 {
	vtable: *mut IDXGIResource1Vtbl
}
#[repr(C)] pub struct IDXGISurface {
	vtable: *mut IDXGISurfaceVtbl
}
#[repr(C)] pub struct IDXGISurface1 {
	vtable: *mut IDXGISurface1Vtbl
}
#[repr(C)] pub struct IDXGISurface2 {
	vtable: *mut IDXGISurface2Vtbl
}
#[repr(C)] pub struct IDXGISwapChain {
	vtable: *mut IDXGISwapChainVtbl
}
#[repr(C)] pub struct IDXGISwapChain1 {
	vtable: *mut IDXGISwapChain1Vtbl
}
#[repr(C)] pub struct IDXGISwapChain2 {
	vtable: *mut IDXGISwapChain2Vtbl
}
#[repr(C)] pub struct IDXGISwapChainMedia {
	vtable: *mut IDXGISwapChainMediaVtbl
}

#[repr(C)] c_vtable!(
IDXGIObjectVtbl of IDXGIObject {
	fn QueryInterface(riid: REFIID, object: *mut *mut c_void) -> HRESULT,
	fn AddRef() -> ULONG,
	fn Release() -> ULONG,
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
		IDXGIAdapter1Vtbl of IDXGIAdapter1
	];
	IDXGIFactoryVtbl of IDXGIFactory {
		fn EnumAdapters(adapter_i: UINT, adapter: *mut *mut IDXGIAdapter) -> HRESULT,
		fn MakeWindowAssociation(window_handle: HWND, flags: UINT) -> HRESULT,
		fn GetWindowAssociation(window_handle: *mut HWND) -> HRESULT,
		fn CreateSwapChain(device: *mut IUnknown, desc: *mut DXGI_SWAP_CHAIN_DESC,
			swapchain: *mut *mut IDXGISwapChain) -> HRESULT,
		fn CreateSoftwareAdapter(module: HMODULE, adapter: *mut *mut IDXGIAdapter) -> HRESULT,
	};
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
	};
]);