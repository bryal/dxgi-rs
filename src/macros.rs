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
macro_rules! define_guid {
	($name:ident, $d1:expr, $d2:expr, $d3:expr, $d4:expr) => {
		pub const $name: GUID = GUID{ Data1: $d1, Data2: $d2, Data3: $d3, Data4: $d4 };
	}
}

macro_rules! c_vtable_struct {
	( ) => { };

	( $tablename:ident, $parent:ty, ) => { pub struct $tablename; };

	( $tablename:ident, $parent:ty,
		$(fn $methodname:ident($($argname:ident: $argtype:ty),*) -> $rettype:ty,)+ ) =>
	{
		pub struct $tablename {
			$( pub $methodname: Option<unsafe extern "system" fn(
				this: *mut $parent, $($argname: $argtype),*) -> $rettype>, )*
		}
	}
}

macro_rules! c_vtable_actual {
	([ ]; $($x:tt)*) => {};

	([ $tablename:ident of $parent:ty { $($methods:tt)* } with heirs [ $($heirs:tt)* ]
			$($siblings:tt)*
		];
		$($inh_methods:tt)* ) =>
	{
		c_vtable_struct!($tablename, $parent, $($inh_methods)* $($methods)* );

		c_vtable_actual!( [ $($heirs)* ]; $($inh_methods)* $($methods)* );
		c_vtable_actual!( [ $($siblings)* ]; $($inh_methods)* );
	};

	([ $tablename:ident of $parent:ty { $($methods:tt)* } $($siblings:tt)* ];
		$($inh_methods:tt)*
	) => {
		c_vtable_actual!( [ $tablename of $parent { $($methods)* } with heirs [ ] $($siblings)* ];
			$($inh_methods)* );
	};
}

macro_rules! c_vtable {
	($($table:tt)*) => {
		c_vtable_actual!([ $($table)* ];);
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
	OneTable of One {
		fn one(a: u32) -> u32,
	} with heirs [
		ZeroTable of Zero {
			fn zero(a: u32) -> u32,
		}
		TwoTable of Two {
			fn two(a: u32) -> u32,
		} with heirs [
			ThreeTable of Three {
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