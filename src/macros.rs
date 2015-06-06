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

//! Simplify interfacing to C vtable representation of C++ classes that make use of inheritance

#![macro_use]
#![allow(dead_code, non_snake_case)]

/// Rust equivalent to windows C DEFINE_GUID macro
#[macro_export]
macro_rules! define_guid {
	($name:ident, $d1:expr, $d2:expr, $d3:expr,
		$d4:expr, $d5:expr, $d6:expr, $d7:expr, $d8:expr, $d9:expr, $d10:expr, $d11:expr) =>
	{
		pub const $name: GUID = GUID{ Data1: $d1, Data2: $d2, Data3: $d3,
			Data4: [$d4, $d5, $d6, $d7, $d8, $d9, $d10, $d11] };
	}
}

#[macro_export]
macro_rules! interface {
	(
		$interface:ident ($vtbl:ident) {$(
			fn $method:ident(&mut self $(,$arg:ident : $argty:ty)*) -> $retty:ty
		),*}
	) => {
		#[repr(C)]
		pub struct $vtbl {
			$(pub $method:
				Option<unsafe extern "system" fn(This: *mut $interface $(,$arg: $argty)*) -> $retty>
			),*
		}
		#[repr(C)]
		pub struct $interface {
			pub lpVtbl: *const $vtbl
		}
		impl $interface {
			$(pub unsafe fn $method(&mut self $(,$arg: $argty)*) -> $retty {
				((*self.lpVtbl).$method.unwrap())(self $(,$arg)*)
			})*
		}
	};

	(
		$interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) {$(
			fn $method:ident(&mut self $(,$arg:ident : $argty:ty)*) -> $retty:ty
		),*}
	) => {
		#[repr(C)]
		pub struct $vtbl {
			pub parent: $pvtbl,
			$(pub $method:
				Option<unsafe extern "system" fn(This: *mut $interface $(,$arg: $argty)*) -> $retty>
			),*
		}
		#[repr(C)]
		pub struct $interface {
			pub lpVtbl: *const $vtbl
		}
		impl $interface {
			$(pub unsafe fn $method(&mut self $(,$arg: $argty)*) -> $retty {
				((*self.lpVtbl).$method.unwrap())(self $(,$arg)*)
			})*
		}
		impl ::std::ops::Deref for $interface {
			type Target = $pinterface;
			fn deref(&self) -> &$pinterface {
				unsafe { ::std::mem::transmute(self) }
			}
		}
		impl ::std::ops::DerefMut for $interface {
			fn deref_mut(&mut self) -> &mut $pinterface {
				unsafe { ::std::mem::transmute(self) }
			}
		}
	};
}

#[macro_export]
macro_rules! com_interface {
	(
		$interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) {$(
			fn $method:ident(&mut self $(,$arg:ident : $argty:ty)*) -> $retty:ty
		),*}
	) => {
		#[repr(C)]
		pub struct $vtbl {
			pub parent: $pvtbl,
			$(pub $method:
				Option<unsafe extern "system" fn(This: *mut $interface $(,$arg: $argty)*) -> $retty>
			),*
		}
		#[repr(C)]
		pub struct $interface {
			pub lpVtbl: *const $vtbl
		}
		impl $interface {
			$(pub unsafe fn $method(&mut self $(,$arg: $argty)*) -> $retty {
				((*self.lpVtbl).$method.unwrap())(self $(,$arg)*)
			})*
		}
		impl ::std::ops::Deref for $interface {
			type Target = $pinterface;
			fn deref(&self) -> &$pinterface {
				unsafe { ::std::mem::transmute(self) }
			}
		}
		impl ::std::ops::DerefMut for $interface {
			fn deref_mut(&mut self) -> &mut $pinterface {
				unsafe { ::std::mem::transmute(self) }
			}
		}
		impl COMInterface for $interface {
			fn i_unknown(&self) -> &IUnknown { &*self }
			fn i_unknown_mut(&mut self) -> &mut IUnknown { &mut *self }
		}
	};
}

#[cfg(test)]
mod test {
	interface!{ One(OneTable) {
		fn one(&mut self, a: u32) -> u32
	}}
	interface!{ Zero(ZeroTable): One(OneTable) {
		fn zero(&mut self, a: u32) -> u32
	}}
	interface!{ Two(TwoTable): One(OneTable) {
		fn two(&mut self, a: u32) -> u32
	}}
	interface!{ Three(ThreeTable): Two(TwoTable) {
		fn three(&mut self, a: u32) -> u32
	}}
}
