mod bb_features;
mod fn_features;
mod llvm_sys_wrapper;
mod mod_features;

use pyo3::{prelude::*, types::PyBytes};

use bb_features::BBFeatures;
use fn_features::FnFeatures;
use llvm_sys_wrapper::{bitcode_to_ir, run_inline_pass};
use mod_features::ModFeatures;

#[pyfunction]
fn llvm_inline_pass<'py>(py: Python<'py>, bc: Bound<'_, PyBytes>) -> PyResult<Bound<'py, PyBytes>> {
    let bc = bc.as_bytes();
    let ret = run_inline_pass(bc);
    match ret {
        Ok(bc) => Ok(PyBytes::new(py, &bc)),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e)),
    }
}

#[pyfunction]
fn bc_to_ir(bc: Bound<PyBytes>) -> PyResult<String> {
    let bc = bc.as_bytes();
    let ret = bitcode_to_ir(bc);
    match ret {
        Ok(ir) => Ok(ir),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e)),
    }
}

#[pymodule]
fn llvm_tools(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(llvm_inline_pass, &m)?)?;
    m.add_function(wrap_pyfunction!(bc_to_ir, &m)?)?;
    m.add_class::<ModFeatures>()?;
    m.add_class::<FnFeatures>()?;
    m.add_class::<BBFeatures>()?;
    Ok(())
}
