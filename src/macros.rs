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
//!
//! # Examples
//! ```
//! 
//! ```

#![macro_use]
#![allow(dead_code)]

/// Rust equivalent to windows C DEFINE_GUID macro
macro_rules! define_guid {
	($name:ident, $d1:expr, $d2:expr, $d3:expr, $d4:expr) => {
		pub static $name: GUID = GUID{ Data1: $d1, Data2: $d2, Data3: $d3, Data4: $d4 };
	}
}

macro_rules! c_vtable_struct {
	( ) => { };

	( $tablename:ident, $parent:ty, ) => { struct $tablename; };

	( $tablename:ident, $parent:ty,
		$(fn $methodname:ident($($argname:ident: $argtype:ty),*) -> $rettype:ty,)+ ) =>
	{
		struct $tablename {
			$( $methodname: Option<unsafe extern "system" fn(
			this: *mut $parent, $($argname: $argtype),*) -> $rettype>, )*
		}
	}
}

macro_rules! c_vtable_actual {
	([ ]; $($x:tt)*) => {};

	([ $tablename:ident of $parent:ty { $($methods:tt)* } with heirs [ $($heirs:tt)* ]
			$($rest:tt)*
		];
		$($inh_methods:tt)*
	) => {
		c_vtable_struct!($tablename, $parent, $($methods)* $($inh_methods)*);

		c_vtable_actual!( [ $($heirs)* ]; $($methods)* $($inh_methods)* );
		c_vtable_actual!( [ $($rest)* ]; $($inh_methods)* );
	};

	([ $tablename:ident of $parent:ty { $($methods:tt)* } $($rest:tt)* ];
		$($inh_methods:tt)*
	) => {
		c_vtable_struct!($tablename, $parent, $($methods)* $($inh_methods)*);

		c_vtable_actual!( [ $($rest)* ]; $($inh_methods)* );
	};
}

macro_rules! c_vtable {
	($($table:tt)*) => {
		c_vtable_actual!([ $($table)* ];);
	}
}

#[cfg(test)]
mod test {
	struct One {
		table: OneTable,
	}

	struct Zero {
		table: ZeroTable,
	}

	struct Three {
		table: ThreeTable,
	}

	struct Two {
		table: TwoTable,
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
	fn three_table() {
		let mut three = Three{
			table: ThreeTable{
				one: Some(fone),
				two: Some(ftwo),
				three: Some(fthree) } };
		let p_three: *mut _ = &mut three;
		let v = 1;
		assert_eq!(unsafe { three.table.one.unwrap()(p_three, v) +
			three.table.three.unwrap()(p_three, v)
		}, 1 + 3);
	}
}