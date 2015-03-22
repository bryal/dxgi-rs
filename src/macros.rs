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
#![allow(dead_code)]

/// Rust equivalent to windows C DEFINE_GUID macro
#[macro_export]
macro_rules! define_guid {
	($name:ident, $d1:expr, $d2:expr, $d3:expr, $d4:expr) => {
		pub const $name: GUID = GUID{ Data1: $d1, Data2: $d2, Data3: $d3, Data4: $d4 };
	}
}

#[macro_export]
macro_rules! c_vtable_struct {
	( ) => { };

	( ($($tpub:ident)*), $tablename:ident, $parent:ty, ) => { #[repr(C)] $($tpub)* struct $tablename; };

	( ($($tpub:ident)*), $tablename:ident, $parent:ty,
		$(fn $methodname:ident($($argname:ident: $argtype:ty),*) -> $rettype:ty,)+ ) =>
	{
		#[repr(C)] $($tpub)* struct $tablename {
			$( pub $methodname: Option<unsafe extern "system" fn(
				this: *mut $parent, $($argname: $argtype),*) -> $rettype>, )*
		}
	}
}

#[macro_export]
macro_rules! c_vtable_methods_to_trait {
	( ($($tpub:ident)*), $traitname:ident, [ ], ) =>
	{
		$($tpub)* trait $traitname { }
	};

	( ($($tpub:ident)*), $traitname:ident, [ ],
		$(fn $methodname:ident($($argname:ident: $argtype:ty),*) -> $rettype:ty,)+ ) =>
	{
		$($tpub)* trait $traitname {
			$(fn $methodname(&mut self, $($argname: $argtype),*) -> $rettype;)*
		}
	};

	( ($($tpub:ident)*), $traitname:ident, [ $parent_trait:ident ],
		$(fn $methodname:ident($($argname:ident: $argtype:ty),*) -> $rettype:ty,)+ ) =>
	{
		$($tpub)* trait $traitname: $parent_trait {
			$(fn $methodname(&mut self, $($argname: $argtype),*) -> $rettype;)*
		}
	};

	( ($($tpub:ident)*), $traitname:ident, [ $parent_trait:ident, $($parent_traits:ident),+ ],
		$(fn $methodname:ident($($argname:ident: $argtype:ty),*) -> $rettype:ty,)+ ) =>
	{
		$($tpub)* trait $traitname: $($parent_traits + )* $parent_trait {
			$(fn $methodname(&mut self, $($argname: $argtype),*) -> $rettype;)*
		}
	};
}

#[macro_export]
macro_rules! impl_c_vtable_trait {
	($traitname:ident, $parent:ty,
		$(fn $methodname:ident($($argname:ident: $argtype:ty),*) -> $rettype:ty,)+ ) =>
	{
		impl $traitname for $parent {
			$(
				fn $methodname(&mut self, $($argname: $argtype),*) -> $rettype {
					unsafe { c_mtdcall!(self,->$methodname($($argname),*)) }
				}
			)*
		}
	};
}

/// Handle the `pub` keyword for vtable and trait, and "with heirs" phrase before main expansion
#[macro_export]
macro_rules! c_vtable_pre {
	([ $table:ident of $parent:ty, trait $traitname:ident { $($methods:tt)* }
			with heirs [$($heirs:tt)*] $($siblings:tt)* ]; $($inheritance:tt)*) =>
	{
		c_vtable_main!((), (), $table, $parent, $traitname, { $($methods)* }, [$($heirs)*],
			[ $($siblings)* ], $($inheritance)*);
	};
	([ pub $table:ident of $parent:ty, trait $traitname:ident { $($methods:tt)* }
			with heirs [$($heirs:tt)*] $($siblings:tt)* ]; $($inheritance:tt)*) =>
	{
		c_vtable_main!((pub), (), $table, $parent, $traitname, { $($methods)* }, [$($heirs)*],
			[ $($siblings)* ], $($inheritance)*);
	};
	([ $table:ident of $parent:ty, pub trait $traitname:ident { $($methods:tt)* }
			with heirs [$($heirs:tt)*] $($siblings:tt)* ]; $($inheritance:tt)*) =>
	{
		c_vtable_main!((), (pub), $table, $parent, $traitname, { $($methods)* }, [$($heirs)*],
			[ $($siblings)* ], $($inheritance)*);
	};
	([ pub $table:ident of $parent:ty, pub trait $traitname:ident { $($methods:tt)* }
			with heirs [$($heirs:tt)*] $($siblings:tt)* ]; $($inheritance:tt)*) =>
	{
		c_vtable_main!((pub), (pub), $table, $parent, $traitname, { $($methods)* }, [$($heirs)*],
			[ $($siblings)* ], $($inheritance)*);
	};

	// No "with heirs"
	([ $table:ident of $parent:ty, trait $traitname:ident { $($methods:tt)* } $($siblings:tt)* ];
		$($inheritance:tt)*) =>
	{
		c_vtable_main!((), (), $table, $parent, $traitname, { $($methods)* }, [ ],
			[ $($siblings)* ], $($inheritance)*);
	};
	([ pub $table:ident of $parent:ty, trait $traitname:ident { $($methods:tt)* }
		$($siblings:tt)* ]; $($inheritance:tt)*) =>
	{
		c_vtable_main!((pub), (), $table, $parent, $traitname, { $($methods)* }, [ ],
			[ $($siblings)* ], $($inheritance)*);
	};
	([ $table:ident of $parent:ty, pub trait $traitname:ident { $($methods:tt)* }
		$($siblings:tt)* ]; $($inheritance:tt)*) =>
	{
		c_vtable_main!((), (pub), $table, $parent, $traitname, { $($methods)* }, [ ],
			[ $($siblings)* ], $($inheritance)*);
	};
	([ pub $table:ident of $parent:ty, pub trait $traitname:ident { $($methods:tt)* }
		$($siblings:tt)* ]; $($inheritance:tt)*) =>
	{
		c_vtable_main!((pub), (pub), $table, $parent, $traitname, { $($methods)* }, [ ],
			[ $($siblings)* ], $($inheritance)*);
	};

	([ ]; $($x:tt)*) => { }
}

#[macro_export]
macro_rules! c_vtable_main {
	(($($tablepub:tt)*), ($($traitpub:tt)*), $table:ident, $parent:ty, $traitname:ident,
		{ $($methods:tt)* }, [$($heirs:tt)*], [ $($siblings:tt)* ],
		$(($inh_trait:ident, $($inh_methods:tt)*))*) =>
	{
		c_vtable_struct!(($($tablepub)*), $table, $parent, $($($inh_methods)*)* $($methods)* );
		c_vtable_methods_to_trait!(($($traitpub)*), $traitname, [ $($inh_trait),* ], $($methods)*);

		impl_c_vtable_trait!($traitname, $parent, $($methods)*);
		$(
			impl_c_vtable_trait!($inh_trait, $parent, $($inh_methods)*);
		)*

		c_vtable_pre!([ $($heirs)* ];
			$(($inh_trait, $($inh_methods)*))* ($traitname, $($methods)*));
		c_vtable_pre!([ $($siblings)* ]; $(($inh_trait, $($inh_methods)*))*);
	};
}

/// Macro used to make making bindings for C representations of C++ classes more convenient.
///
/// This macro, through `c_vtable_actual!`, also provides traits to make interfacing with rust code
/// easier.
///
/// This:
///
/// ```rust,ignore
/// c_vtable!(
/// IUnknownVtbl of IUnknown, trait IUnknownT { 
///     fn QueryInterface(riid: REFIID, object: *mut *mut c_void) -> HRESULT,
///     fn AddRef() -> ULONG,
///     fn Release() -> ULONG,
/// } with heirs [
///     IDXGIDebugVtbl of IDXGIDebug, trait IDXGIDebugT {
///         fn ReportLiveObjects(apiid: GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> HRESULT,
///     }
/// ]);
/// ```
///
/// Expands to this:
///
/// ```rust,ignore
/// #[repr(C)]
/// pub struct IUnknownVtbl {
///     pub QueryInterface: Option<unsafe extern "system" fn(this: *mut IUnknown, riid: REFIID,
///         object: *mut *mut c_void) -> HRESULT>,
///     pub AddRef: Option<unsafe extern "system" fn(this: *mut IUnknown) -> ULONG>,
///     pub Release: Option<unsafe extern "system" fn(this: *mut IUnknown) -> ULONG>,
/// }
/// trait IUnknownT {
///     fn QueryInterface(&mut self, riid: REFIID, object: *mut *mut c_void) -> HRESULT;
///     fn AddRef(&mut self) -> ULONG;
///     fn Release(&mut self) -> ULONG;
/// }
/// impl IUnknownT for IUnknown {
///     fn QueryInterface(&mut self, riid: REFIID, object: *mut *mut c_void) -> HRESULT {
///         unsafe {
///             (*(*self).vtable).QueryInterface.unwrap()(&mut *self, riid, object)
///         }
///     }
///     fn AddRef(&mut self) -> ULONG {
///         unsafe { (*(*self).vtable).AddRef.unwrap()(&mut *self) }
///     }
///     fn Release(&mut self) -> ULONG {
///         unsafe { (*(*self).vtable).Release.unwrap()(&mut *self) }
///     }
/// }
/// #[repr(C)]
/// pub struct IDXGIDebugVtbl {
///     pub QueryInterface: Option<unsafe extern "system" fn(this: *mut IDXGIDebug, riid: REFIID,
///         object: *mut *mut c_void) -> HRESULT>,
///     pub AddRef: Option<unsafe extern "system" fn(this: *mut IDXGIDebug) -> ULONG>,
///     pub Release: Option<unsafe extern "system" fn(this: *mut IDXGIDebug) -> ULONG>,
///     pub ReportLiveObjects: Option<unsafe extern "system" fn(this: *mut IDXGIDebug,
///         apiid: GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> HRESULT>,
/// }
/// trait IDXGIDebugT: IUnknownT {
///     fn ReportLiveObjects(&mut self, apiid: GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> HRESULT;
/// }
/// impl IDXGIDebugT for IDXGIDebug {
///     fn ReportLiveObjects(&mut self, apiid: GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> HRESULT {
///         unsafe {
///             (*(*self).vtable).ReportLiveObjects.unwrap()(&mut *self, apiid, flags)
///         }
///     }
/// }
/// impl IUnknownT for IDXGIDebug {
///     fn QueryInterface(&mut self, riid: REFIID, object: *mut *mut c_void) -> HRESULT {
///         unsafe {
///             (*(*self).vtable).QueryInterface.unwrap()(&mut *self, riid, object)
///         }
///     }
///     fn AddRef(&mut self) -> ULONG {
///         unsafe { (*(*self).vtable).AddRef.unwrap()(&mut *self) }
///     }
///     fn Release(&mut self) -> ULONG {
///         unsafe { (*(*self).vtable).Release.unwrap()(&mut *self) }
///     }
/// }
/// ```
#[macro_export]
macro_rules! c_vtable {
	($($table:tt)*) => {
		c_vtable_pre!([ $($table)* ];);
	}
}

/// Call a method of a C++ object represented as a C struct
#[macro_export]
macro_rules! c_mtdcall {
	( $obj:expr,->$method:ident( ) ) => {
		(*(*$obj).vtable).$method.unwrap()(&mut *$obj)
	};

	( $obj:expr,->$method:ident($($args:expr),+) ) => {
		(*(*$obj).vtable).$method.unwrap()(&mut *$obj, $($args),*)
	};

	( $obj:expr,.$method:ident( ) ) => {
		(*$obj.vtable).$method.unwrap()(&mut $obj)
	};

	( $obj:expr,.$method:ident($($args:expr),+) ) => {
		(*$obj.vtable).$method.unwrap()(&mut $obj, $($args),*)
	};
}

#[cfg(test)]
mod test {
	struct One {
		vtable: *mut OneTable,
	}

	struct Zero {
		vtable: *mut ZeroTable,
	}

	struct Three {
		vtable: *mut ThreeTable,
	}

	struct Two {
		vtable: *mut TwoTable,
	}

	// Create the vtables using inheritance
	#[repr(C)] c_vtable!(
	OneTable of One, trait OneT {
		fn one(a: u32) -> u32,
	} with heirs [
		ZeroTable of Zero, trait ZeroT {
			fn zero(a: u32) -> u32,
		}
		TwoTable of Two, trait TwoT {
			fn two(a: u32) -> u32,
		} with heirs [
			ThreeTable of Three, trait ThreeT {
				fn three(a: u32) -> u32,
			}
		]
	]);

	// Current funtion pointer members:
	// OneTable => one
	// ZeroTable => one, zero
	// TwoTable => one, two
	// ThreeTable => one, two, three

	extern "system" fn fone(_: *mut Three, a: u32) -> u32 { a * 1 }
	extern "system" fn ftwo(_: *mut Three, a: u32) -> u32 { a * 2 }
	extern "system" fn fthree(_: *mut Three, a: u32) -> u32 { a * 3 }

	#[test]
	fn c_method_call_macro() {
		extern "system" fn fone(_: *mut One, _: u32) -> u32 { 1337 }
		let one: *mut _ = &mut One{
			vtable: &mut OneTable{
				one: Some(fone) } };

		unsafe { assert_eq!(c_mtdcall!(one,->one(1)), (*(*one).vtable).one.unwrap()(one, 1)); }
	}

	#[test]
	fn three_table() {
		let mut three = Three{
			vtable: &mut ThreeTable{
				one: Some(fone),
				two: Some(ftwo),
				three: Some(fthree) } };
		let v = 1;
		assert_eq!(unsafe { c_mtdcall!(three,.one(v)) + c_mtdcall!(three,.three(v)) }, 1 + 3);
	}
}