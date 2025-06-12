mod mod_features;
mod fn_features;
mod bb_features;

use pyo3::prelude::*;

use mod_features::ModFeatures;
use fn_features::FnFeatures;
use bb_features::BBFeatures;

#[pymodule]
fn llvm_tools(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ModFeatures>()?;
    m.add_class::<FnFeatures>()?;
    m.add_class::<BBFeatures>()?;
    Ok(())
}
