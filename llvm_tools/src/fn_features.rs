use std::collections::HashMap;

use llvm_ir::{function::FunctionAttribute, Function};
use pyo3::{pyclass, pymethods, PyResult};
use rayon::prelude::*;

use crate::bb_features::BBFeatures;

#[pyclass]
#[derive(Clone)]
/// Features of a function
pub struct FnFeatures {
    _function: Function,

    #[pyo3(get)]
    /// The name of the function
    pub name: String,
    #[pyo3(get)]
    /// The basic blocks in the function
    pub bb_feats: HashMap<String, BBFeatures>,
    #[pyo3(get)]
    /// The number of arguments the function takes
    pub arg_count: usize,
    #[pyo3(get)]
    /// The number of instructions in the function
    pub instruction_count: usize,
}

impl FnFeatures {
    pub fn new(function: &Function) -> Self {
        let name = function.name.to_string();
        let bb_feats: HashMap<String, BBFeatures> = function
            .basic_blocks
            .par_iter()
            .map(|bb| {
                let bb_feat = BBFeatures::new(bb);
                (bb_feat.name.clone(), bb_feat)
            })
            .collect();
        let arg_count = function.parameters.len();
        let instruction_count = bb_feats
            .values()
            .map(|bb| bb.instruction_count)
            .sum::<usize>();
        Self {
            _function: function.clone(),
            name,
            bb_feats,
            arg_count,
            instruction_count,
        }
    }
}

#[pymethods]
impl FnFeatures {
    /// Whether the function has variable arguments
    pub fn has_var_args(&self) -> PyResult<bool> {
        Ok(self._function.is_var_arg)
    }

    /// Whether the function has the `inlinehint` attribute
    pub fn has_inline_hint(&self) -> PyResult<bool> {
        Ok(self
            ._function
            .function_attributes
            .contains(&FunctionAttribute::InlineHint))
    }

    /// The number of basic blocks in the function
    pub fn bb_count(&self) -> PyResult<usize> {
        Ok(self.bb_feats.len())
    }

    /// Whether the function has the `alwaysinline` attribute
    pub fn has_always_inline(&self) -> PyResult<bool> {
        Ok(self
            ._function
            .function_attributes
            .contains(&FunctionAttribute::AlwaysInline))
    }

    /// Whether the function has the `noinline` attribute
    pub fn has_no_inline(&self) -> PyResult<bool> {
        Ok(self
            ._function
            .function_attributes
            .contains(&FunctionAttribute::NoInline))
    }
}
