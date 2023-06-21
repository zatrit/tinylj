use libc::{c_uchar, c_int};
use super::lua::lua_State;

pub const LUA_FILEHANDLE: &[c_uchar] = b"FILE*\x00";
pub const LUA_COLIBNAME: &[c_uchar] = b"coroutine\x00";
pub const LUA_MATHLIBNAME: &[c_uchar] = b"math\x00";
pub const LUA_STRLIBNAME: &[c_uchar] = b"string\x00";
pub const LUA_TABLIBNAME: &[c_uchar] = b"table\x00";
pub const LUA_IOLIBNAME: &[c_uchar] = b"io\x00";
pub const LUA_OSLIBNAME: &[c_uchar] = b"os\x00";
pub const LUA_LOADLIBNAME: &[c_uchar] = b"package\x00";
pub const LUA_DBLIBNAME: &[c_uchar] = b"debug\x00";
pub const LUA_BITLIBNAME: &[c_uchar] = b"bit\x00";
pub const LUA_JITLIBNAME: &[c_uchar] = b"jit\x00";
pub const LUA_FFILIBNAME: &[c_uchar] = b"ffi\x00";

extern "C" {
    pub fn luaopen_base(L: *mut lua_State) -> c_int;
    pub fn luaopen_math(L: *mut lua_State) -> c_int;
    pub fn luaopen_string(L: *mut lua_State) -> c_int;
    pub fn luaopen_table(L: *mut lua_State) -> c_int;
    pub fn luaopen_io(L: *mut lua_State) -> c_int;
    pub fn luaopen_os(L: *mut lua_State) -> c_int;
    pub fn luaopen_package(L: *mut lua_State) -> c_int;
    pub fn luaopen_debug(L: *mut lua_State) -> c_int;
    pub fn luaopen_bit(L: *mut lua_State) -> c_int;
    pub fn luaopen_jit(L: *mut lua_State) -> c_int;
    pub fn luaopen_ffi(L: *mut lua_State) -> c_int;
    pub fn luaL_openlibs(L: *mut lua_State);
}