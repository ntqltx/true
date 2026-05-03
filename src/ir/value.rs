use std::ptr::slice_from_raw_parts_mut;

const QNAN: u64 = 0x7FFC_0000_0000_0000;
const SIGN_BIT: u64 = 0x8000_0000_0000_0000;
const TAG_NIL: u64 = 1;
const TAG_FALSE: u64 = 2;
const TAG_TRUE: u64 = 3;
const PTR_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Value(pub u64);

impl Value {
	pub const NIL: Self = Self(QNAN | TAG_NIL);
	pub const TRUE: Self = Self(QNAN | TAG_TRUE);
	pub const FALSE: Self = Self(QNAN | TAG_FALSE);

	pub fn number(n: f64) -> Self {
		Self(n.to_bits())
	}

	pub fn bool(b: bool) -> Self {
		if b { Self::TRUE } else { Self::FALSE }
	}

	pub fn obj(p: *mut Obj) -> Self {
		Self(SIGN_BIT | QNAN | (p as u64))
	}

	pub fn is_obj(self) -> bool {
		(self.0 & (SIGN_BIT | QNAN)) == (SIGN_BIT | QNAN)
	}

	pub fn as_obj(self) -> *mut Obj {
		(self.0 & PTR_MASK) as *mut Obj
	}
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ObjKind {
	String = 0,
}

#[repr(C)]
pub struct Obj {
	pub kind: ObjKind,
}

#[repr(C)]
pub struct ObjString {
	pub obj: Obj,
	pub ptr: *mut u8,
	pub len: usize,
}

pub fn alloc_obj_string(s: &str) -> *mut Obj {
	let bytes = s.as_bytes().to_vec().into_boxed_slice();
	let len = bytes.len();
	let ptr = Box::into_raw(bytes) as *mut u8;

	Box::into_raw(Box::new(ObjString {
		obj: Obj { kind: ObjKind::String },
		ptr,
		len,
	})) as *mut Obj
}

pub unsafe fn obj_free(obj: *mut Obj) {
	if obj.is_null() {
		return;
	}

	unsafe {
		match (*obj).kind {
			ObjKind::String => {
				let s = obj as *mut ObjString;
				drop(Box::from_raw(slice_from_raw_parts_mut((*s).ptr, (*s).len)));
				drop(Box::from_raw(s));
			}
		}
	}
}
