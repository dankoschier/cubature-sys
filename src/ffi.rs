use ::std::os::raw::{c_int, c_uint, c_void};

#[repr(C)]
pub enum ErrorNorm {
    Individual = 0,
    Paired = 1,
    L2 = 2,
    L1 = 3,
    LInf = 4,
}

pub type Integrand = ::std::option::Option<
    unsafe extern "C" fn(
        ndim: c_uint,
        x: *const f64,
        arg1: *mut c_void,
        fdim: c_uint,
        fval: *mut f64,
    ) -> c_int,
>;

pub type IntegrandV = ::std::option::Option<
    unsafe extern "C" fn(
        ndim: c_uint,
        npt: usize,
        x: *const f64,
        arg1: *mut c_void,
        fdim: c_uint,
        fval: *mut f64,
    ) -> c_int,
>;

unsafe extern "C" {

    pub fn hcubature(
        fdim: c_uint,
        f: Integrand,
        fdata: *mut c_void,
        dim: c_uint,
        xmin: *const f64,
        xmax: *const f64,
        maxEval: usize,
        reqAbsError: f64,
        reqRelError: f64,
        norm: ErrorNorm,
        val: *mut f64,
        err: *mut f64,
    ) -> c_int;

    pub fn hcubature_v(
        fdim: c_uint,
        f: IntegrandV,
        fdata: *mut c_void,
        dim: c_uint,
        xmin: *const f64,
        xmax: *const f64,
        maxEval: usize,
        reqAbsError: f64,
        reqRelError: f64,
        norm: ErrorNorm,
        val: *mut f64,
        err: *mut f64,
    ) -> c_int;

    pub fn pcubature(
        fdim: c_uint,
        f: Integrand,
        fdata: *mut c_void,
        dim: c_uint,
        xmin: *const f64,
        xmax: *const f64,
        maxEval: usize,
        reqAbsError: f64,
        reqRelError: f64,
        norm: ErrorNorm,
        val: *mut f64,
        err: *mut f64,
    ) -> c_int;

    pub fn pcubature_v(
        fdim: c_uint,
        f: IntegrandV,
        fdata: *mut c_void,
        dim: c_uint,
        xmin: *const f64,
        xmax: *const f64,
        maxEval: usize,
        reqAbsError: f64,
        reqRelError: f64,
        norm: ErrorNorm,
        val: *mut f64,
        err: *mut f64,
    ) -> c_int;

    pub fn pcubature_v_buf(
        fdim: c_uint,
        f: IntegrandV,
        fdata: *mut c_void,
        dim: c_uint,
        xmin: *const f64,
        xmax: *const f64,
        maxEval: usize,
        reqAbsError: f64,
        reqRelError: f64,
        norm: ErrorNorm,
        m: *mut c_uint,
        buf: *mut *mut f64,
        nbuf: *mut usize,
        max_nbuf: usize,
        val: *mut f64,
        err: *mut f64,
    ) -> c_int;

}
