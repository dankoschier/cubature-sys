pub mod ffi;
pub use crate::ffi::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::addr_of_mut;

    #[test]
    fn h_integrate_sawtooth() {
        extern "C" fn my_integrand(
            ndim: ::std::os::raw::c_uint,
            x: *const f64,
            arg1: *mut ::std::os::raw::c_void,
            fdim: ::std::os::raw::c_uint,
            fval: *mut f64,
        ) -> ::std::os::raw::c_int {
            unsafe {
                let xv = std::slice::from_raw_parts(x, ndim as usize);
                let fvalv = std::slice::from_raw_parts_mut(fval, fdim as usize);
                fvalv[0] = xv[0] - xv[0].floor();
                fvalv[1] = xv[1] - xv[1].floor();
                fvalv[2] = (xv[0] - xv[0].floor()) * (xv[1] - xv[1].floor());
                *arg1.cast::<usize>() += 1;
            }
            0
        }

        let xmin = [0.0, 0.0];
        let xmax = [1.5, 1.5];
        let mut num_eval: usize = 0;
        let max_eval = 100000;
        let req_abs_error = 1.0e-8;
        let req_rel_error = 1.0e-8;
        let mut valv: [f64; 3] = Default::default();
        let mut errv: [f64; 3] = Default::default();
        unsafe {
            hcubature(
                3,
                Some(my_integrand),
                addr_of_mut!(num_eval) as *mut _,
                2,
                xmin.as_ptr(),
                xmax.as_ptr(),
                max_eval,
                req_abs_error,
                req_rel_error,
                ErrorNorm::L2,
                valv.as_mut_ptr(),
                errv.as_mut_ptr(),
            );
        }
        assert!(num_eval < max_eval);

        let expected_vals = [0.9375, 0.9375, 0.390625];
        for ((&val, &err), &expected_val) in valv.iter().zip(errv.iter()).zip(expected_vals.iter())
        {
            assert!((val - expected_val).abs() < req_abs_error);
            assert!(err < req_abs_error);
        }
    }
}
