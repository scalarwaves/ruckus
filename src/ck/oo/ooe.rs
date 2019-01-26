#![allow(
    dead_code,
    improper_ctypes,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports,
    unused_mut
)]
#![feature(libc)]
//* Base classes and objects for the ChucK runtime VM implementation.
extern crate libc;
use crate::chuck_carrier_h_edited::*;
use crate::chuck_def_h_edited::*;
use crate::chuck_type_h_edited::*;
use crate::dts::*;
use crate::util_thread_h_edited::*;
use std::mem::*;
use std::option::Option;
use std::os::raw::*;
use std::prelude::v1::*;
pub const CHUCK_ARRAY4_DATAKIND: u32 = 1;
pub const CHUCK_ARRAY8_DATAKIND: u32 = 2;
pub const CHUCK_ARRAY16_DATAKIND: u32 = 3;
pub const CHUCK_ARRAY24_DATAKIND: u32 = 4;
pub const CHUCK_ARRAY32_DATAKIND: u32 = 5;
pub type error_t = libc::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ChucK {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Compiler {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_VM {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Env {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ck_socket_ {
    _unused: [u8; 0],
}
pub type ck_socket = *mut ck_socket_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WvOut {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct Chuck_Carrier {
    pub chuck: *mut ChucK,
    pub compiler: *mut Chuck_Compiler,
    pub env: *mut Chuck_Env,
    pub vm: *mut Chuck_VM,
    pub chout: *mut Chuck_IO_Chout,
    pub cherr: *mut Chuck_IO_Cherr,
    pub otf_socket: ck_socket,
    pub otf_port: libc::c_long,
    pub otf_thread: pthread_t,
    pub stk_writeThread: *mut XWriteThread,
    pub stk_wvOutMap: std::collections::HashMap::new(),
}
extern "C" {
    #[link_name = "\u{1}hintIsRealtimeAudio"]
    pub fn Chuck_Carrier_hintIsRealtimeAudio(this: *mut Chuck_Carrier) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}Chuck_Carrier"]
    pub fn Chuck_Carrier_Chuck_Carrier(this: *mut Chuck_Carrier);
}
impl Chuck_Carrier {
    #[inline]
    pub unsafe fn hintIsRealtimeAudio(&mut self) -> libc::c_ulong {
        Chuck_Carrier_hintIsRealtimeAudio(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Carrier_Chuck_Carrier(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Type {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Value {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Func {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Namespace {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_Context {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_VM_Code {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Chuck_VM_Shred {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CBufferSimple {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct Chuck_VM_Object__bindgen_vtable(libc::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_VM_Object {
    pub vtable_: *const Chuck_VM_Object__bindgen_vtable,
    pub m_ref_count: libc::c_ulong,
    pub m_pooled: libc::c_ulong,
    pub m_locked: libc::c_ulong,
    pub m_v_ref: *mut crate::dts::vector,
}
extern "C" {
    #[link_name = "\u{1}our_locks_in_effect"]
    pub static mut Chuck_VM_Object_our_locks_in_effect: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}lock_all"]
    pub fn Chuck_VM_Object_lock_all();
}
extern "C" {
    #[link_name = "\u{1}unlock_all"]
    pub fn Chuck_VM_Object_unlock_all();
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Object"]
    pub fn Chuck_VM_Object_Chuck_VM_Object(this: *mut Chuck_VM_Object);
}
impl Chuck_VM_Object {
    #[inline]
    pub unsafe fn lock_all() {
        Chuck_VM_Object_lock_all()
    }
    #[inline]
    pub unsafe fn unlock_all() {
        Chuck_VM_Object_unlock_all()
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_VM_Object_Chuck_VM_Object(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_VM_Object_destructor"]
    pub fn Chuck_VM_Object_Chuck_VM_Object_destructor(this: *mut Chuck_VM_Object);
}
extern "C" {
    #[link_name = "\u{1}add_ref"]
    pub fn Chuck_VM_Object_add_ref(this: *mut libc::c_void);
}
extern "C" {
    #[link_name = "\u{1}release"]
    pub fn Chuck_VM_Object_release(this: *mut libc::c_void);
}
extern "C" {
    #[link_name = "\u{1}lock"]
    pub fn Chuck_VM_Object_lock(this: *mut libc::c_void);
}
#[repr(C)]
pub struct Chuck_VTable {
    pub funcs: crate::dts::vector,
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Object {
    pub _base: Chuck_VM_Object,
    pub vtable: *mut Chuck_VTable,
    pub type_ref: *mut Chuck_Type,
    pub size: libc::c_ulong,
    pub data: *mut libc::c_uchar,
}
extern "C" {
    #[link_name = "\u{1}Chuck_Object"]
    pub fn Chuck_Object_Chuck_Object(this: *mut Chuck_Object);
}
impl Chuck_Object {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Object_Chuck_Object(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Object_destructor"]
    pub fn Chuck_Object_Chuck_Object_destructor(this: *mut Chuck_Object);
}
#[repr(C)]
#[derive(Debug)]
pub struct Chuck_Array {
    pub _base: Chuck_Object,
    pub m_array_type: *mut Chuck_Type,
}
#[repr(C)]
pub struct Chuck_Array4 {
    pub _base: Chuck_Array,
    pub m_vector: crate::dts::vector,
    pub m_map: std::collections::HashMap::new(),
    pub m_is_obj: libc::c_ulong,
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array4_addr(this: *mut Chuck_Array4, i: libc::c_long) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array4_addr1(
        this: *mut Chuck_Array4,
        key: *const crate::dts::string,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array4_get(
        this: *mut Chuck_Array4,
        i: libc::c_long,
        val: *mut libc::c_ulong,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array4_get1(
        this: *mut Chuck_Array4,
        key: *const crate::dts::string,
        val: *mut libc::c_ulong,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array4_set(
        this: *mut Chuck_Array4,
        i: libc::c_long,
        val: libc::c_ulong,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array4_set1(
        this: *mut Chuck_Array4,
        key: *const crate::dts::string,
        val: libc::c_ulong,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array4_push_back(this: *mut Chuck_Array4, val: libc::c_ulong) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array4_pop_back(this: *mut Chuck_Array4) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array4_back(this: *const Chuck_Array4, val: *mut libc::c_ulong) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array4_zero(this: *mut Chuck_Array4, start: libc::c_ulong, end: libc::c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array4"]
    pub fn Chuck_Array4_Chuck_Array4(
        this: *mut Chuck_Array4,
        is_obj: libc::c_ulong,
        capacity: libc::c_long,
    );
}
impl Chuck_Array4 {
    #[inline]
    pub unsafe fn addr(&mut self, i: libc::c_long) -> libc::c_ulong {
        Chuck_Array4_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const crate::dts::string) -> libc::c_ulong {
        Chuck_Array4_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(&mut self, i: libc::c_long, val: *mut libc::c_ulong) -> libc::c_long {
        Chuck_Array4_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(
        &mut self,
        key: *const crate::dts::string,
        val: *mut libc::c_ulong,
    ) -> libc::c_long {
        Chuck_Array4_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(&mut self, i: libc::c_long, val: libc::c_ulong) -> libc::c_long {
        Chuck_Array4_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(
        &mut self,
        key: *const crate::dts::string,
        val: libc::c_ulong,
    ) -> libc::c_long {
        Chuck_Array4_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: libc::c_ulong) -> libc::c_long {
        Chuck_Array4_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> libc::c_long {
        Chuck_Array4_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut libc::c_ulong) -> libc::c_long {
        Chuck_Array4_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: libc::c_ulong, end: libc::c_ulong) {
        Chuck_Array4_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(is_obj: libc::c_ulong, capacity: libc::c_long) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Array4_Chuck_Array4(&mut __bindgen_tmp, is_obj, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array4_destructor"]
    pub fn Chuck_Array4_Chuck_Array4_destructor(this: *mut Chuck_Array4);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_Array4_clear(this: *mut libc::c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_Array4_size(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    pub fn Chuck_Array4_capacity(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    pub fn Chuck_Array4_set_size(this: *mut libc::c_void, size: libc::c_long) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    pub fn Chuck_Array4_set_capacity(
        this: *mut libc::c_void,
        capacity: libc::c_long,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    pub fn Chuck_Array4_find(
        this: *mut libc::c_void,
        key: *const crate::dts::string,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array4_erase(
        this: *mut libc::c_void,
        key: *const crate::dts::string,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    pub fn Chuck_Array4_data_type_size(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    pub fn Chuck_Array4_data_type_kind(this: *mut libc::c_void) -> libc::c_long;
}
#[repr(C)]
pub struct Chuck_Array8 {
    pub _base: Chuck_Array,
    pub m_vector: crate::dts::vector,
    pub m_map: std::collections::HashMap::new(),
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array8_addr(this: *mut Chuck_Array8, i: libc::c_long) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array8_addr1(
        this: *mut Chuck_Array8,
        key: *const crate::dts::string,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array8_get(
        this: *mut Chuck_Array8,
        i: libc::c_long,
        val: *mut f64,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array8_get1(
        this: *mut Chuck_Array8,
        key: *const crate::dts::string,
        val: *mut f64,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array8_set(this: *mut Chuck_Array8, i: libc::c_long, val: f64) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array8_set1(
        this: *mut Chuck_Array8,
        key: *const crate::dts::string,
        val: f64,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array8_push_back(this: *mut Chuck_Array8, val: f64) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array8_pop_back(this: *mut Chuck_Array8) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array8_back(this: *const Chuck_Array8, val: *mut f64) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array8_zero(this: *mut Chuck_Array8, start: libc::c_ulong, end: libc::c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array8"]
    pub fn Chuck_Array8_Chuck_Array8(this: *mut Chuck_Array8, capacity: libc::c_long);
}
impl Chuck_Array8 {
    #[inline]
    pub unsafe fn addr(&mut self, i: libc::c_long) -> libc::c_ulong {
        Chuck_Array8_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const crate::dts::string) -> libc::c_ulong {
        Chuck_Array8_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(&mut self, i: libc::c_long, val: *mut f64) -> libc::c_long {
        Chuck_Array8_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(
        &mut self,
        key: *const crate::dts::string,
        val: *mut f64,
    ) -> libc::c_long {
        Chuck_Array8_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(&mut self, i: libc::c_long, val: f64) -> libc::c_long {
        Chuck_Array8_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(
        &mut self,
        key: *const crate::dts::string,
        val: f64,
    ) -> libc::c_long {
        Chuck_Array8_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: f64) -> libc::c_long {
        Chuck_Array8_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> libc::c_long {
        Chuck_Array8_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut f64) -> libc::c_long {
        Chuck_Array8_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: libc::c_ulong, end: libc::c_ulong) {
        Chuck_Array8_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(capacity: libc::c_long) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Array8_Chuck_Array8(&mut __bindgen_tmp, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array8_destructor"]
    pub fn Chuck_Array8_Chuck_Array8_destructor(this: *mut Chuck_Array8);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_Array8_clear(this: *mut libc::c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_Array8_size(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    pub fn Chuck_Array8_capacity(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    pub fn Chuck_Array8_set_size(this: *mut libc::c_void, size: libc::c_long) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    pub fn Chuck_Array8_set_capacity(
        this: *mut libc::c_void,
        capacity: libc::c_long,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    pub fn Chuck_Array8_find(
        this: *mut libc::c_void,
        key: *const crate::dts::string,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array8_erase(
        this: *mut libc::c_void,
        key: *const crate::dts::string,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    pub fn Chuck_Array8_data_type_size(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    pub fn Chuck_Array8_data_type_kind(this: *mut libc::c_void) -> libc::c_long;
}
#[repr(C)]
pub struct Chuck_Array16 {
    pub _base: Chuck_Array,
    pub m_vector: crate::dts::vector,
    pub m_map: std::collections::HashMap::new(),
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array16_addr(this: *mut Chuck_Array16, i: libc::c_long) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array16_addr1(
        this: *mut Chuck_Array16,
        key: *const crate::dts::string,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array16_get(
        this: *mut Chuck_Array16,
        i: libc::c_long,
        val: *mut t_CKCOMPLEX,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array16_get1(
        this: *mut Chuck_Array16,
        key: *const crate::dts::string,
        val: *mut t_CKCOMPLEX,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array16_set(
        this: *mut Chuck_Array16,
        i: libc::c_long,
        val: *const t_CKCOMPLEX,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array16_set1(
        this: *mut Chuck_Array16,
        key: *const crate::dts::string,
        val: *const t_CKCOMPLEX,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array16_push_back(
        this: *mut Chuck_Array16,
        val: *const t_CKCOMPLEX,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array16_pop_back(this: *mut Chuck_Array16) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array16_back(this: *const Chuck_Array16, val: *mut t_CKCOMPLEX) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array16_zero(this: *mut Chuck_Array16, start: libc::c_ulong, end: libc::c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array16"]
    pub fn Chuck_Array16_Chuck_Array16(this: *mut Chuck_Array16, capacity: libc::c_long);
}
impl Chuck_Array16 {
    #[inline]
    pub unsafe fn addr(&mut self, i: libc::c_long) -> libc::c_ulong {
        Chuck_Array16_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const crate::dts::string) -> libc::c_ulong {
        Chuck_Array16_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(&mut self, i: libc::c_long, val: *mut t_CKCOMPLEX) -> libc::c_long {
        Chuck_Array16_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(
        &mut self,
        key: *const crate::dts::string,
        val: *mut t_CKCOMPLEX,
    ) -> libc::c_long {
        Chuck_Array16_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(&mut self, i: libc::c_long, val: *const t_CKCOMPLEX) -> libc::c_long {
        Chuck_Array16_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(
        &mut self,
        key: *const crate::dts::string,
        val: *const t_CKCOMPLEX,
    ) -> libc::c_long {
        Chuck_Array16_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: *const t_CKCOMPLEX) -> libc::c_long {
        Chuck_Array16_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> libc::c_long {
        Chuck_Array16_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut t_CKCOMPLEX) -> libc::c_long {
        Chuck_Array16_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: libc::c_ulong, end: libc::c_ulong) {
        Chuck_Array16_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(capacity: libc::c_long) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Array16_Chuck_Array16(&mut __bindgen_tmp, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array16_destructor"]
    pub fn Chuck_Array16_Chuck_Array16_destructor(this: *mut Chuck_Array16);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_Array16_clear(this: *mut libc::c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_Array16_size(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    pub fn Chuck_Array16_capacity(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    pub fn Chuck_Array16_set_size(this: *mut libc::c_void, size: libc::c_long) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    pub fn Chuck_Array16_set_capacity(
        this: *mut libc::c_void,
        capacity: libc::c_long,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    pub fn Chuck_Array16_find(
        this: *mut libc::c_void,
        key: *const crate::dts::string,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array16_erase(
        this: *mut libc::c_void,
        key: *const crate::dts::string,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    pub fn Chuck_Array16_data_type_size(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    pub fn Chuck_Array16_data_type_kind(this: *mut libc::c_void) -> libc::c_long;
}
#[repr(C)]
pub struct Chuck_Array24 {
    pub _base: Chuck_Array,
    pub m_vector: crate::dts::vector,
    pub m_map: std::collections::HashMap::new(),
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array24_addr(this: *mut Chuck_Array24, i: libc::c_long) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array24_addr1(
        this: *mut Chuck_Array24,
        key: *const crate::dts::string,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array24_get(
        this: *mut Chuck_Array24,
        i: libc::c_long,
        val: *mut t_CKVEC3,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array24_get1(
        this: *mut Chuck_Array24,
        key: *const crate::dts::string,
        val: *mut t_CKVEC3,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array24_set(
        this: *mut Chuck_Array24,
        i: libc::c_long,
        val: *const t_CKVEC3,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array24_set1(
        this: *mut Chuck_Array24,
        key: *const crate::dts::string,
        val: *const t_CKVEC3,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array24_push_back(this: *mut Chuck_Array24, val: *const t_CKVEC3) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array24_pop_back(this: *mut Chuck_Array24) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array24_back(this: *const Chuck_Array24, val: *mut t_CKVEC3) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array24_zero(this: *mut Chuck_Array24, start: libc::c_ulong, end: libc::c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array24"]
    pub fn Chuck_Array24_Chuck_Array24(this: *mut Chuck_Array24, capacity: libc::c_long);
}
impl Chuck_Array24 {
    #[inline]
    pub unsafe fn addr(&mut self, i: libc::c_long) -> libc::c_ulong {
        Chuck_Array24_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const crate::dts::string) -> libc::c_ulong {
        Chuck_Array24_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(&mut self, i: libc::c_long, val: *mut t_CKVEC3) -> libc::c_long {
        Chuck_Array24_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(
        &mut self,
        key: *const crate::dts::string,
        val: *mut t_CKVEC3,
    ) -> libc::c_long {
        Chuck_Array24_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(&mut self, i: libc::c_long, val: *const t_CKVEC3) -> libc::c_long {
        Chuck_Array24_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(
        &mut self,
        key: *const crate::dts::string,
        val: *const t_CKVEC3,
    ) -> libc::c_long {
        Chuck_Array24_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: *const t_CKVEC3) -> libc::c_long {
        Chuck_Array24_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> libc::c_long {
        Chuck_Array24_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut t_CKVEC3) -> libc::c_long {
        Chuck_Array24_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: libc::c_ulong, end: libc::c_ulong) {
        Chuck_Array24_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(capacity: libc::c_long) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Array24_Chuck_Array24(&mut __bindgen_tmp, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array24_destructor"]
    pub fn Chuck_Array24_Chuck_Array24_destructor(this: *mut Chuck_Array24);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_Array24_clear(this: *mut libc::c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_Array24_size(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    pub fn Chuck_Array24_capacity(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    pub fn Chuck_Array24_set_size(this: *mut libc::c_void, size: libc::c_long) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    pub fn Chuck_Array24_set_capacity(
        this: *mut libc::c_void,
        capacity: libc::c_long,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    pub fn Chuck_Array24_find(
        this: *mut libc::c_void,
        key: *const crate::dts::string,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array24_erase(
        this: *mut libc::c_void,
        key: *const crate::dts::string,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    pub fn Chuck_Array24_data_type_size(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    pub fn Chuck_Array24_data_type_kind(this: *mut libc::c_void) -> libc::c_long;
}
#[repr(C)]
pub struct Chuck_Array32 {
    pub _base: Chuck_Array,
    pub m_vector: crate::dts::vector,
    pub m_map: std::collections::HashMap::new(),
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array32_addr(this: *mut Chuck_Array32, i: libc::c_long) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}addr"]
    pub fn Chuck_Array32_addr1(
        this: *mut Chuck_Array32,
        key: *const crate::dts::string,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array32_get(
        this: *mut Chuck_Array32,
        i: libc::c_long,
        val: *mut t_CKVEC4,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}get"]
    pub fn Chuck_Array32_get1(
        this: *mut Chuck_Array32,
        key: *const crate::dts::string,
        val: *mut t_CKVEC4,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array32_set(
        this: *mut Chuck_Array32,
        i: libc::c_long,
        val: *const t_CKVEC4,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_Array32_set1(
        this: *mut Chuck_Array32,
        key: *const crate::dts::string,
        val: *const t_CKVEC4,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}push_back"]
    pub fn Chuck_Array32_push_back(this: *mut Chuck_Array32, val: *const t_CKVEC4) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}pop_back"]
    pub fn Chuck_Array32_pop_back(this: *mut Chuck_Array32) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}back"]
    pub fn Chuck_Array32_back(this: *const Chuck_Array32, val: *mut t_CKVEC4) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}zero"]
    pub fn Chuck_Array32_zero(this: *mut Chuck_Array32, start: libc::c_ulong, end: libc::c_ulong);
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array32"]
    pub fn Chuck_Array32_Chuck_Array32(this: *mut Chuck_Array32, capacity: libc::c_long);
}
impl Chuck_Array32 {
    #[inline]
    pub unsafe fn addr(&mut self, i: libc::c_long) -> libc::c_ulong {
        Chuck_Array32_addr(self, i)
    }
    #[inline]
    pub unsafe fn addr1(&mut self, key: *const crate::dts::string) -> libc::c_ulong {
        Chuck_Array32_addr1(self, key)
    }
    #[inline]
    pub unsafe fn get(&mut self, i: libc::c_long, val: *mut t_CKVEC4) -> libc::c_long {
        Chuck_Array32_get(self, i, val)
    }
    #[inline]
    pub unsafe fn get1(
        &mut self,
        key: *const crate::dts::string,
        val: *mut t_CKVEC4,
    ) -> libc::c_long {
        Chuck_Array32_get1(self, key, val)
    }
    #[inline]
    pub unsafe fn set(&mut self, i: libc::c_long, val: *const t_CKVEC4) -> libc::c_long {
        Chuck_Array32_set(self, i, val)
    }
    #[inline]
    pub unsafe fn set1(
        &mut self,
        key: *const crate::dts::string,
        val: *const t_CKVEC4,
    ) -> libc::c_long {
        Chuck_Array32_set1(self, key, val)
    }
    #[inline]
    pub unsafe fn push_back(&mut self, val: *const t_CKVEC4) -> libc::c_long {
        Chuck_Array32_push_back(self, val)
    }
    #[inline]
    pub unsafe fn pop_back(&mut self) -> libc::c_long {
        Chuck_Array32_pop_back(self)
    }
    #[inline]
    pub unsafe fn back(&self, val: *mut t_CKVEC4) -> libc::c_long {
        Chuck_Array32_back(self, val)
    }
    #[inline]
    pub unsafe fn zero(&mut self, start: libc::c_ulong, end: libc::c_ulong) {
        Chuck_Array32_zero(self, start, end)
    }
    #[inline]
    pub unsafe fn new(capacity: libc::c_long) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_Array32_Chuck_Array32(&mut __bindgen_tmp, capacity);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_Array32_destructor"]
    pub fn Chuck_Array32_Chuck_Array32_destructor(this: *mut Chuck_Array32);
}
extern "C" {
    #[link_name = "\u{1}clear"]
    pub fn Chuck_Array32_clear(this: *mut libc::c_void);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_Array32_size(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}capacity"]
    pub fn Chuck_Array32_capacity(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_size"]
    pub fn Chuck_Array32_set_size(this: *mut libc::c_void, size: libc::c_long) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}set_capacity"]
    pub fn Chuck_Array32_set_capacity(
        this: *mut libc::c_void,
        capacity: libc::c_long,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}find"]
    pub fn Chuck_Array32_find(
        this: *mut libc::c_void,
        key: *const crate::dts::string,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}erase"]
    pub fn Chuck_Array32_erase(
        this: *mut libc::c_void,
        key: *const crate::dts::string,
    ) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_size"]
    pub fn Chuck_Array32_data_type_size(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}data_type_kind"]
    pub fn Chuck_Array32_data_type_kind(this: *mut libc::c_void) -> libc::c_long;
}
#[repr(C)]
pub struct Chuck_Event {
    pub _base: Chuck_Object,
    pub m_queue: crate::dts::deque,
    pub m_queue_lock: XMutex,
}
extern "C" {
    #[link_name = "\u{1}our_can_wait"]
    pub static mut Chuck_Event_our_can_wait: libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}signal"]
    pub fn Chuck_Event_signal(this: *mut Chuck_Event);
}
extern "C" {
    #[link_name = "\u{1}broadcast"]
    pub fn Chuck_Event_broadcast(this: *mut Chuck_Event);
}
extern "C" {
    #[link_name = "\u{1}wait"]
    pub fn Chuck_Event_wait(this: *mut Chuck_Event, shred: *mut Chuck_VM_Shred, vm: *mut Chuck_VM);
}
extern "C" {
    #[link_name = "\u{1}remove"]
    pub fn Chuck_Event_remove(this: *mut Chuck_Event, shred: *mut Chuck_VM_Shred) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}queue_broadcast"]
    pub fn Chuck_Event_queue_broadcast(this: *mut Chuck_Event, event_buffer: *mut CBufferSimple);
}
impl Chuck_Event {
    #[inline]
    pub unsafe fn signal(&mut self) {
        Chuck_Event_signal(self)
    }
    #[inline]
    pub unsafe fn broadcast(&mut self) {
        Chuck_Event_broadcast(self)
    }
    #[inline]
    pub unsafe fn wait(&mut self, shred: *mut Chuck_VM_Shred, vm: *mut Chuck_VM) {
        Chuck_Event_wait(self, shred, vm)
    }
    #[inline]
    pub unsafe fn remove(&mut self, shred: *mut Chuck_VM_Shred) -> libc::c_ulong {
        Chuck_Event_remove(self, shred)
    }
    #[inline]
    pub unsafe fn queue_broadcast(&mut self, event_buffer: *mut CBufferSimple) {
        Chuck_Event_queue_broadcast(self, event_buffer)
    }
}
#[repr(C)]
pub struct Chuck_String {
    pub _base: Chuck_Object,
    pub m_charptr: *const libc::c_char,
    pub m_str: crate::dts::string,
}
extern "C" {
    #[link_name = "\u{1}set"]
    pub fn Chuck_String_set(this: *mut Chuck_String, s: *const crate::dts::string);
}
extern "C" {
    #[link_name = "\u{1}str"]
    pub fn Chuck_String_str(this: *mut Chuck_String) -> *const crate::dts::string;
}
extern "C" {
    #[link_name = "\u{1}c_str"]
    pub fn Chuck_String_c_str(this: *mut Chuck_String) -> *const libc::c_char;
}
extern "C" {
    #[link_name = "\u{1}Chuck_String"]
    pub fn Chuck_String_Chuck_String(
        this: *mut Chuck_String,
        s: *const crate::dts::string,
    );
}
impl Chuck_String {
    #[inline]
    pub unsafe fn set(&mut self, s: *const crate::dts::string) {
        Chuck_String_set(self, s)
    }
    #[inline]
    pub unsafe fn str(&mut self) -> *const crate::dts::string {
        Chuck_String_str(self)
    }
    #[inline]
    pub unsafe fn c_str(&mut self) -> *const libc::c_char {
        Chuck_String_c_str(self)
    }
    #[inline]
    pub unsafe fn new(s: *const crate::dts::string) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_String_Chuck_String(&mut __bindgen_tmp, s);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_String_destructor"]
    pub fn Chuck_String_Chuck_String_destructor(this: *mut Chuck_String);
}
#[repr(C)]
pub struct Chuck_IO {
    pub _base: Chuck_Event,
    pub m_asyncEvent: *mut Chuck_Event,
    pub m_thread: *mut XThread,
}
#[repr(C)]
pub struct Chuck_IO_async_args {
    pub fileio_obj: *mut Chuck_IO_File,
    pub RETURN: *mut libc::c_void,
    pub intArg: libc::c_long,
    pub floatArg: f64,
    pub stringArg: crate::dts::string,
}
extern "C" {
    #[link_name = "\u{1}INT32"]
    pub static Chuck_IO_INT32: libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}INT16"]
    pub static Chuck_IO_INT16: libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}INT8"]
    pub static Chuck_IO_INT8: libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}MODE_SYNC"]
    pub static Chuck_IO_MODE_SYNC: libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}MODE_ASYNC"]
    pub static Chuck_IO_MODE_ASYNC: libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO"]
    pub fn Chuck_IO_Chuck_IO(this: *mut Chuck_IO);
}
impl Chuck_IO {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_IO_Chuck_IO(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_destructor"]
    pub fn Chuck_IO_Chuck_IO_destructor(this: *mut Chuck_IO);
}
#[repr(C)]
pub struct Chuck_IO_File {
    pub _base: Chuck_IO,
    pub m_flags: libc::c_long,
    pub m_iomode: libc::c_long,
    pub m_io: crate::dts::fstream(std::fs::Path::new(&str)),
    pub m_dir: *mut DIR,
    pub m_dir_start: libc::c_long,
    pub m_path: crate::dts::string,
    pub m_vmRef: *mut Chuck_VM,
}
extern "C" {
    #[link_name = "\u{1}FLAG_READ_WRITE"]
    pub static Chuck_IO_File_FLAG_READ_WRITE: libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}FLAG_READONLY"]
    pub static Chuck_IO_File_FLAG_READONLY: libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}FLAG_WRITEONLY"]
    pub static Chuck_IO_File_FLAG_WRITEONLY: libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}FLAG_APPEND"]
    pub static Chuck_IO_File_FLAG_APPEND: libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}TYPE_ASCII"]
    pub static Chuck_IO_File_TYPE_ASCII: libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}TYPE_BINARY"]
    pub static Chuck_IO_File_TYPE_BINARY: libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}writeStr_thread"]
    pub fn Chuck_IO_File_writeStr_thread(data: *mut libc::c_void) -> THREAD_RETURN;
}
extern "C" {
    #[link_name = "\u{1}writeInt_thread"]
    pub fn Chuck_IO_File_writeInt_thread(data: *mut libc::c_void) -> THREAD_RETURN;
}
extern "C" {
    #[link_name = "\u{1}writeFloat_thread"]
    pub fn Chuck_IO_File_writeFloat_thread(data: *mut libc::c_void) -> THREAD_RETURN;
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_File"]
    pub fn Chuck_IO_File_Chuck_IO_File(this: *mut Chuck_IO_File, vm: *mut Chuck_VM);
}
impl Chuck_IO_File {
    #[inline]
    pub unsafe fn writeStr_thread(data: *mut libc::c_void) -> THREAD_RETURN {
        Chuck_IO_File_writeStr_thread(data)
    }
    #[inline]
    pub unsafe fn writeInt_thread(data: *mut libc::c_void) -> THREAD_RETURN {
        Chuck_IO_File_writeInt_thread(data)
    }
    #[inline]
    pub unsafe fn writeFloat_thread(data: *mut libc::c_void) -> THREAD_RETURN {
        Chuck_IO_File_writeFloat_thread(data)
    }
    #[inline]
    pub unsafe fn new(vm: *mut Chuck_VM) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_IO_File_Chuck_IO_File(&mut __bindgen_tmp, vm);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_File_destructor"]
    pub fn Chuck_IO_File_Chuck_IO_File_destructor(this: *mut Chuck_IO_File);
}
extern "C" {
    #[link_name = "\u{1}open"]
    pub fn Chuck_IO_File_open(
        this: *mut libc::c_void,
        path: *const crate::dts::string,
        flags: libc::c_long,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn Chuck_IO_File_good(this: *mut libc::c_void) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn Chuck_IO_File_close(this: *mut libc::c_void);
}
extern "C" {
    #[link_name = "\u{1}flush"]
    pub fn Chuck_IO_File_flush(this: *mut libc::c_void);
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_File_mode(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_File_mode1(this: *mut libc::c_void, flag: libc::c_long);
}
extern "C" {
    #[link_name = "\u{1}size"]
    pub fn Chuck_IO_File_size(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}seek"]
    pub fn Chuck_IO_File_seek(this: *mut libc::c_void, pos: libc::c_long);
}
extern "C" {
    #[link_name = "\u{1}tell"]
    pub fn Chuck_IO_File_tell(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}isDir"]
    pub fn Chuck_IO_File_isDir(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}dirList"]
    pub fn Chuck_IO_File_dirList(this: *mut libc::c_void) -> *mut Chuck_Array4;
}
extern "C" {
    #[link_name = "\u{1}readLine"]
    pub fn Chuck_IO_File_readLine(this: *mut libc::c_void) -> *mut Chuck_String;
}
extern "C" {
    #[link_name = "\u{1}readInt"]
    pub fn Chuck_IO_File_readInt(this: *mut libc::c_void, flags: libc::c_long) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}readFloat"]
    pub fn Chuck_IO_File_readFloat(this: *mut libc::c_void) -> f64;
}
extern "C" {
    #[link_name = "\u{1}readString"]
    pub fn Chuck_IO_File_readString(
        this: *mut libc::c_void,
        str: *mut crate::dts::string,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_File_eof(this: *mut libc::c_void) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_File_write(
        this: *mut libc::c_void,
        val: *const crate::dts::string,
    );
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_File_write1(this: *mut libc::c_void, val: libc::c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_File_write2(this: *mut libc::c_void, val: libc::c_long, flags: libc::c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_File_write3(this: *mut libc::c_void, val: f64);
}
#[repr(C)]
pub struct Chuck_IO_Chout {
    pub _base: Chuck_IO,
    pub m_callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    pub m_buffer: crate::dts::basic_istringstream<_CharT>,
}
extern "C" {
    #[link_name = "\u{1}set_output_callback"]
    pub fn Chuck_IO_Chout_set_output_callback(
        this: *mut Chuck_IO_Chout,
        fp: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Chout"]
    pub fn Chuck_IO_Chout_Chuck_IO_Chout(this: *mut Chuck_IO_Chout, carrier: *mut Chuck_Carrier);
}
impl Chuck_IO_Chout {
    #[inline]
    pub unsafe fn set_output_callback(
        &mut self,
        fp: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    ) {
        Chuck_IO_Chout_set_output_callback(self, fp)
    }
    #[inline]
    pub unsafe fn new(carrier: *mut Chuck_Carrier) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_IO_Chout_Chuck_IO_Chout(&mut __bindgen_tmp, carrier);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Chout_destructor"]
    pub fn Chuck_IO_Chout_Chuck_IO_Chout_destructor(this: *mut Chuck_IO_Chout);
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn Chuck_IO_Chout_good(this: *mut libc::c_void) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn Chuck_IO_Chout_close(this: *mut libc::c_void);
}
extern "C" {
    #[link_name = "\u{1}flush"]
    pub fn Chuck_IO_Chout_flush(this: *mut libc::c_void);
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_Chout_mode(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_Chout_mode1(this: *mut libc::c_void, flag: libc::c_long);
}
extern "C" {
    #[link_name = "\u{1}readLine"]
    pub fn Chuck_IO_Chout_readLine(this: *mut libc::c_void) -> *mut Chuck_String;
}
extern "C" {
    #[link_name = "\u{1}readInt"]
    pub fn Chuck_IO_Chout_readInt(this: *mut libc::c_void, flags: libc::c_long) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}readFloat"]
    pub fn Chuck_IO_Chout_readFloat(this: *mut libc::c_void) -> f64;
}
extern "C" {
    #[link_name = "\u{1}readString"]
    pub fn Chuck_IO_Chout_readString(
        this: *mut libc::c_void,
        str: *mut crate::dts::string,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_Chout_eof(this: *mut libc::c_void) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Chout_write(
        this: *mut libc::c_void,
        val: *const crate::dts::string,
    );
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Chout_write1(this: *mut libc::c_void, val: libc::c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Chout_write2(this: *mut libc::c_void, val: libc::c_long, flags: libc::c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Chout_write3(this: *mut libc::c_void, val: f64);
}
#[repr(C)]
pub struct Chuck_IO_Cherr {
    pub _base: Chuck_IO,
    pub m_callback: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    pub m_buffer: crate::dts::basic_istringstream<_CharT>,
}
extern "C" {
    #[link_name = "\u{1}set_output_callback"]
    pub fn Chuck_IO_Cherr_set_output_callback(
        this: *mut Chuck_IO_Cherr,
        fp: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    );
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Cherr"]
    pub fn Chuck_IO_Cherr_Chuck_IO_Cherr(this: *mut Chuck_IO_Cherr, carrier: *mut Chuck_Carrier);
}
impl Chuck_IO_Cherr {
    #[inline]
    pub unsafe fn set_output_callback(
        &mut self,
        fp: ::std::option::Option<unsafe extern "C" fn(arg1: *const libc::c_char)>,
    ) {
        Chuck_IO_Cherr_set_output_callback(self, fp)
    }
    #[inline]
    pub unsafe fn new(carrier: *mut Chuck_Carrier) -> Self {
        let mut __bindgen_tmp = uninitialized();
        Chuck_IO_Cherr_Chuck_IO_Cherr(&mut __bindgen_tmp, carrier);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}Chuck_IO_Cherr_destructor"]
    pub fn Chuck_IO_Cherr_Chuck_IO_Cherr_destructor(this: *mut Chuck_IO_Cherr);
}
extern "C" {
    #[link_name = "\u{1}good"]
    pub fn Chuck_IO_Cherr_good(this: *mut libc::c_void) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}close"]
    pub fn Chuck_IO_Cherr_close(this: *mut libc::c_void);
}
extern "C" {
    #[link_name = "\u{1}flush"]
    pub fn Chuck_IO_Cherr_flush(this: *mut libc::c_void);
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_Cherr_mode(this: *mut libc::c_void) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}mode"]
    pub fn Chuck_IO_Cherr_mode1(this: *mut libc::c_void, flag: libc::c_long);
}
extern "C" {
    #[link_name = "\u{1}readLine"]
    pub fn Chuck_IO_Cherr_readLine(this: *mut libc::c_void) -> *mut Chuck_String;
}
extern "C" {
    #[link_name = "\u{1}readInt"]
    pub fn Chuck_IO_Cherr_readInt(this: *mut libc::c_void, flags: libc::c_long) -> libc::c_long;
}
extern "C" {
    #[link_name = "\u{1}readFloat"]
    pub fn Chuck_IO_Cherr_readFloat(this: *mut libc::c_void) -> f64;
}
extern "C" {
    #[link_name = "\u{1}readString"]
    pub fn Chuck_IO_Cherr_readString(
        this: *mut libc::c_void,
        str: *mut crate::dts::string,
    ) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}eof"]
    pub fn Chuck_IO_Cherr_eof(this: *mut libc::c_void) -> libc::c_ulong;
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Cherr_write(
        this: *mut libc::c_void,
        val: *const crate::dts::string,
    );
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Cherr_write1(this: *mut libc::c_void, val: libc::c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Cherr_write2(this: *mut libc::c_void, val: libc::c_long, flags: libc::c_long);
}
extern "C" {
    #[link_name = "\u{1}write"]
    pub fn Chuck_IO_Cherr_write3(this: *mut libc::c_void, val: f64);
}