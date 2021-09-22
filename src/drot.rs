/// Applies an `f64` plane rotation to 2 _n_-element `f64` vectors: `x` and `y`, with respective strides `incx` and `incy`.
/// 
/// <link rel="stylesheet"
/// href="https://cdn.jsdelivr.net/npm/katex@0.10.0/dist/katex.min.css"
/// integrity="sha384-9eLZqc9ds8eNjO3TmqPeYcDj8n+Qfa4nuSiGYa6DjLNcv9BtN69ZIulL9+8CqC9Y"
/// crossorigin="anonymous">
/// <script src="https://cdn.jsdelivr.net/npm/katex@0.10.0/dist/katex.min.js"
///   integrity="sha384-K3vbOmF2BtaVai+Qk37uypf7VrgBubhQreNQe9aGsz9lB63dIFiQVlJbr92dw2Lx"
///   crossorigin="anonymous"></script>
/// <script src="https://cdn.jsdelivr.net/npm/katex@0.10.0/dist/contrib/auto-render.min.js"
///   integrity="sha384-kmZOZB5ObwgQnS/DuDg6TScgOiWWBiVt0plIRkZCmE6rDZGrEOQeHM5PcHi+nyqe"
///   crossorigin="anonymous"></script>
/// <script>
/// document.addEventListener("DOMContentLoaded", function() {
///   renderMathInElement(document.body, {
///       delimiters: [
///           {left: "$$", right: "$$", display: true},
///           {left: "\\(", right: "\\)", display: false},
///           {left: "$", right: "$", display: false},
///           {left: "\\[", right: "\\]", display: true}
///       ]
///   });
/// });
/// </script>
/// 
/// $$ \\begin{bmatrix}
///      x^\\prime \\\\ y^\\prime
///    \\end{bmatrix} \leftarrow
///    \\begin{bmatrix}
///      c & s \\\\
///     -s & c
///    \\end{bmatrix}
///    \\cdot
///    \\begin{bmatrix}
///      x \\\\ y
///    \\end{bmatrix}
/// $$
/// 
/// - `n: isize`<br>Number of planar points, in `x` and `y`, to be rotated.
///   - _on entry_:. if `n = 0`, this function returns immediately.
/// 
/// - `x: &mut [f64]`<br>Array of dimension at least `(n - 1) * incx + 1`.
///    - _on entry_: the _n_-elements are `x[i * incx] for i = 0..n`
///    - _on exit_: the rotated values are updated in-place.
/// 
/// - `incx: isize`<br>Increment between elements of `x` as input and output.
///   - _on entry_: if `incx = 0`, this function returns immediately.
/// 
/// - `y: &mut [f64]`<br>Array of dimension at least `(n - 1) * incy + 1`.
///   - _on entry_: the _n_-elements are `y[i * incy] for i = 0..n`
///   - _on exit_: the rotated values are updated in-place.
/// 
/// - `incy: isize`<br>Increment between elements of `y` as input and output.
///   - _on entry_: if `incy = 0`, this function returns immediately.
/// 
/// - `c: f64`<br>Cosine of the angle of rotation.
/// 
/// - `s: f64`<br>Sine of the angle of rotation.
/// 
/// If coefficients `c` and `s` satisfy $c^2+s^2=1$, the rotation matrix is orthogonal, and the transformation is called a Givens plane rotation.
/// 
/// If `c = 1` and `s = 0`, this function returns immediately.
/// 
/// Reference:
/// 1. [https://www.hpc.nec/documents/sdk/SDK_NLC/UsersGuide/man/drot.html](https://www.hpc.nec/documents/sdk/SDK_NLC/UsersGuide/man/drot.html)
pub fn drot(n: isize, x: &mut [f64], incx: isize, y: &mut [f64], incy: isize, c: f64, s: f64) -> bool {
    if n <= 0 {
        return true;
    }

    if c == 1.0 && s == 0.0 {
        return true;
    }

    if incx > 0 {
        if x.len() < 1 + ((n as usize) - 1) * (incx as usize) {
            return false;
        }
    }
    if incx < 0 {
        if x.len() < 1 + ((n as usize) - 1) * ((-incx) as usize) {
            return false;
        }
    }

    if incy > 0 {
        if y.len() < 1 + ((n as usize) - 1) * (incy as usize) {
            return false;
        }
    }
    if incy < 0 {
        if y.len() < 1 + ((n as usize) - 1) * ((-incy) as usize) {
            return false;
        }
    }

    let n_usize = n as usize;
    if incx == 1 && incy == 1 {
        for i in 0 .. n_usize {
            let temp = c * x[i] + s * y[i];
            y[i] = c * y[i] - s * x[i];
            x[i] = temp;
        }
        return true;
    }

    let incx_abs: usize;
    let mut ix: usize = if incx < 0 {
        incx_abs = (-incx) as usize;
        ((-incx) as usize) * (n_usize - 1)
    } else {
        incx_abs = incx as usize;
        0_usize
    };
    
    let incy_abs: usize;
    let mut iy: usize = if incy < 0 {
        incy_abs = (-incy) as usize;
        ((-incy) as usize) * (n_usize - 1)
    } else {
        incy_abs = incy as usize;
        0_usize
    };

    for _ in 0 .. n_usize {
        let temp = c * x[ix] + s * y[iy];
        y[iy] = c * y[iy] - s * x[ix];
        x[ix] = temp;
        
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