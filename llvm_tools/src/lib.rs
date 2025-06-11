mod llvm_module;
mod bb_stats;
mod function_stats;
mod module_stats;

use pyo3::{prelude::*, types::PyBytes};

use llvm_module::LLVMModule;
use bb_stats::BBStats;
use function_stats::FunctionStats;
use module_stats::ModuleStats;

#[pyfunction]
fn bitcode_to_ir(bitcode: Bound<PyBytes>) -> PyResult<String> {
    let module = LLVMModule::from_bitcode(bitcode.as_bytes())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    let ir = module
        .to_ir()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    Ok(ir)
}

#[pymodule]
fn llvm_tools(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(bitcode_to_ir))?;
    m.add_class::<BBStats>()?;
    m.add_class::<FunctionStats>()?;
    m.add_class::<ModuleStats>()?;
    Ok(())
}
