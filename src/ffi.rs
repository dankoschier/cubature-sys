/* automatically generated by rust-bindgen 0.71.1 */

pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of max_align_t"][::std::mem::size_of::<max_align_t>() - 32usize];
    ["Alignment of max_align_t"][::std::mem::align_of::<max_align_t>() - 16usize];
    ["Offset of field: max_align_t::__clang_max_align_nonce1"]
        [::std::mem::offset_of!(max_align_t, __clang_max_align_nonce1) - 0usize];
    ["Offset of field: max_align_t::__clang_max_align_nonce2"]
        [::std::mem::offset_of!(max_align_t, __clang_max_align_nonce2) - 16usize];
};
pub type integrand = ::std::option::Option<
    unsafe extern "C" fn(
        ndim: ::std::os::raw::c_uint,
        x: *const f64,
        arg1: *mut ::std::os::raw::c_void,
        fdim: ::std::os::raw::c_uint,
        fval: *mut f64,
    ) -> ::std::os::raw::c_int,
>;
pub type integrand_v = ::std::option::Option<
    unsafe extern "C" fn(
        ndim: ::std::os::raw::c_uint,
        npt: usize,
        x: *const f64,
        arg1: *mut ::std::os::raw::c_void,
        fdim: ::std::os::raw::c_uint,
        fval: *mut f64,
    ) -> ::std::os::raw::c_int,
>;
pub const error_norm_ERROR_INDIVIDUAL: error_norm = 0;
pub const error_norm_ERROR_PAIRED: error_norm = 1;
pub const error_norm_ERROR_L2: error_norm = 2;
pub const error_norm_ERROR_L1: error_norm = 3;
pub const error_norm_ERROR_LINF: error_norm = 4;
pub type error_norm = ::std::os::raw::c_uint;
unsafe extern "C" {
    pub fn hcubature(
        fdim: ::std::os::raw::c_uint,
        f: integrand,
        fdata: *mut ::std::os::raw::c_void,
        dim: ::std::os::raw::c_uint,
        xmin: *const f64,
        xmax: *const f64,
        maxEval: usize,
        reqAbsError: f64,
        reqRelError: f64,
        norm: error_norm,
        val: *mut f64,
        err: *mut f64,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn hcubature_v(
        fdim: ::std::os::raw::c_uint,
        f: integrand_v,
        fdata: *mut ::std::os::raw::c_void,
        dim: ::std::os::raw::c_uint,
        xmin: *const f64,
        xmax: *const f64,
        maxEval: usize,
        reqAbsError: f64,
        reqRelError: f64,
        norm: error_norm,
        val: *mut f64,
        err: *mut f64,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pcubature_v_buf(
        fdim: ::std::os::raw::c_uint,
        f: integrand_v,
        fdata: *mut ::std::os::raw::c_void,
        dim: ::std::os::raw::c_uint,
        xmin: *const f64,
        xmax: *const f64,
        maxEval: usize,
        reqAbsError: f64,
        reqRelError: f64,
        norm: error_norm,
        m: *mut ::std::os::raw::c_uint,
        buf: *mut *mut f64,
        nbuf: *mut usize,
        max_nbuf: usize,
        val: *mut f64,
        err: *mut f64,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pcubature_v(
        fdim: ::std::os::raw::c_uint,
        f: integrand_v,
        fdata: *mut ::std::os::raw::c_void,
        dim: ::std::os::raw::c_uint,
        xmin: *const f64,
        xmax: *const f64,
        maxEval: usize,
        reqAbsError: f64,
        reqRelError: f64,
        norm: error_norm,
        val: *mut f64,
        err: *mut f64,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pcubature(
        fdim: ::std::os::raw::c_uint,
        f: integrand,
        fdata: *mut ::std::os::raw::c_void,
        dim: ::std::os::raw::c_uint,
        xmin: *const f64,
        xmax: *const f64,
        maxEval: usize,
        reqAbsError: f64,
        reqRelError: f64,
        norm: error_norm,
        val: *mut f64,
        err: *mut f64,
    ) -> ::std::os::raw::c_int;
}