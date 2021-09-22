use crate::utils::{check_inc, get_first_index};

fn copy<T>(n: isize, x: &[T], incx: isize, y: &mut [T], incy: isize) -> bool
where T: Copy,
{
    if check_inc(n, x, incx) != true || check_inc(n, y, incy) != true {
        return false;
    }

    let n_usize = n as usize;
    if incx == 1 && incy == 1 {
        for i in 0 .. n_usize {
            y[i] = x[i];
        }
        return true;
    }

    let incx_abs: usize = if incx > 0 {incx as usize} else {(-incx) as usize};
    let mut ix: usize = get_first_index(n_usize, incx);
    
    let incy_abs: usize = if incy > 0 {incy as usize} else {(-incy) as usize};
    let mut iy: usize = get_first_index(n_usize, incy);

    for _ in 0 .. n_usize {
        y[iy] = x[ix];
        
        ix = if incx > 0 {
            ix + incx_abs
        } else {
            ix - incx_abs
        };
        iy = if incy > 0 {
            iy + incy_abs
        } else {
            iy - incy_abs
        };
    }

    true
}

/// copies a `f32` vector into another `f32` vector
pub fn scopy(n: isize, x: &[f32], incx: isize, y: &mut [f32], incy: isize) -> bool {
    copy::<f32>(n, x, incx, y, incy)
}

/// copies a `f64` vector into another `f64` vector
pub fn dcopy(n: isize, x: &[f64], incx: isize, y: &mut [f64], incy: isize) -> bool {
    copy::<f64>(n, x, incx, y, incy)
}