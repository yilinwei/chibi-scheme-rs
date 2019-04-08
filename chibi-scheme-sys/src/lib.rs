#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::os::raw;

const fn sexp_make_immediate(n: u32) -> sexp {
    ((n << SEXP_EXTENDED_BITS) + SEXP_EXTENDED_TAG) as sexp
}

pub fn sexp_unbox_fixnum(n: sexp) -> sexp_sint_t {
    (n as sexp_sint_t) >> SEXP_FIXNUM_BITS
}

pub fn sexp_make_character(n: raw::c_char) -> sexp {
    (((n as sexp_sint_t) << SEXP_EXTENDED_BITS) + (SEXP_CHAR_TAG as sexp_sint_t)) as sexp
}

pub fn sexp_unbox_character(n: sexp) -> raw::c_char {
    ((n as sexp_sint_t) >> SEXP_EXTENDED_BITS) as raw::c_char
}

pub const SEXP_FALSE: sexp = sexp_make_immediate(0);
pub const SEXP_TRUE: sexp = sexp_make_immediate(1);
pub const SEXP_NULL: sexp = sexp_make_immediate(2);
pub const SEXP_EOF: sexp = sexp_make_immediate(3);
pub const SEXP_VOID: sexp = sexp_make_immediate(4);

pub fn sexp_truep(x: sexp) -> bool {
    x != SEXP_FALSE
}

pub fn sexp_not(x: sexp) -> bool {
    x == SEXP_FALSE
}

pub fn sexp_nullp(x: sexp) -> bool {
    x == SEXP_NULL
}

pub fn sexp_fixnump(x: sexp) -> bool {
    ((x as sexp_uint_t) & SEXP_FIXNUM_MASK as sexp_uint_t) == SEXP_FIXNUM_TAG as sexp_uint_t
}

pub fn sexp_isymbolp(x: sexp) -> bool {
    ((x as sexp_uint_t) & SEXP_IMMEDIATE_MASK as sexp_uint_t) == SEXP_ISYMBOL_TAG as sexp_uint_t
}

pub fn sexp_charp(x: sexp) -> bool {
    ((x as sexp_uint_t) & SEXP_EXTENDED_MASK as sexp_uint_t) == SEXP_CHAR_TAG as sexp_uint_t
}

pub fn sexp_booleanp(x: sexp) -> bool {
    x == SEXP_TRUE || x == SEXP_FALSE
}

pub fn sexp_pointerp(x: sexp) -> bool {
    ((x as sexp_uint_t) & SEXP_POINTER_MASK as sexp_uint_t) == SEXP_POINTER_TAG as sexp_uint_t
}

pub fn sexp_pointer_tag(x: sexp) -> sexp_tag_t {
    unsafe { (*x).tag }
}

pub fn sexp_check_tag(x: sexp, t: sexp_tag_t) -> bool {
    sexp_pointerp(x) &&
        (sexp_pointer_tag(x) == t)
}

pub fn sexp_stringp(x: sexp) -> bool {
    sexp_check_tag(x, sexp_types_SEXP_STRING)
}

pub fn sexp_string_size(x: sexp) -> sexp_uint_t {
    unsafe { (*x).value.string.as_ref().length }
}

pub fn sexp_bytes_length(x: sexp) -> sexp_uint_t {
    unsafe { (*x).value.bytes.as_ref().length }
}

pub fn sexp_bytes_data(x: sexp) -> *const raw::c_char {
    unsafe { (*x).value.bytes.as_ref().data.as_ptr() }
}

pub fn sexp_string_offset(x: sexp) -> sexp_uint_t {
    unsafe { (*x).value.string.as_ref().offset }
}

pub fn sexp_string_data(x: sexp) -> *const raw::c_char {
    unsafe{ sexp_bytes_data((*x).value.string.as_ref().bytes).offset(sexp_string_offset(x) as isize) }
}

pub fn sexp_string_length(x: sexp) -> sexp_uint_t {
    unsafe{ (*x).value.string.as_ref().length }
}

// TODO: Safe accessor
// TODO: Add feature for stuff
