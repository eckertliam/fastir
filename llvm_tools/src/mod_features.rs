use std::collections::{HashMap, HashSet};

use llvm_ir::Module;
use pyo3::{prelude::*, pyclass, pymethods, types::PyBytes, Bound, PyResult};
use rayon::prelude::*;

use crate::{fn_features::FnFeatures, llvm_sys_wrapper::run_inline_pass};

#[pyclass]
pub struct ModFeatures {
    #[pyo3(get)]
    pub fn_feats: HashMap<String, FnFeatures>,
    #[pyo3(get)]
    /// (caller_name, bb_name, callee_name)
    pub call_sites: HashSet<(String, String, String)>,
}

impl ModFeatures {
    fn from_bc(bc: &[u8]) -> Result<Self, String> {
        let module = Module::from_bc_bytes(bc)?;
        // get fn definitions
        let mut fn_feats: HashMap<String, FnFeatures> = module
            .functions
            .par_iter()
            .map(|func| {
                let stats = FnFeatures::from_def(&func);
                (stats.name.clone(), stats)
            })
            .collect();
        // get fn declarations
        fn_feats.extend(module.func_declarations.iter().map(|decl| {
            let stats = FnFeatures::from_declaration(&decl);
            (stats.name.clone(), stats)
        }));
        let call_sites = fn_feats
            .values()
            .flat_map(|func| {
                func.calls
                    .iter()
                    .map(|(bb_name, callee_name)| (func.name.clone(), bb_name.clone(), callee_name.clone()))
            })
            .collect();
        Ok(Self {
            fn_feats,
            call_sites,
        })
    }

    pub fn inlined_mod_features(bc: &[u8]) -> Result<Self, String> {
        let inlined_bc = run_inline_pass(bc)?;
        Self::from_bc(&inlined_bc)
    }
}

#[pymethods]
impl ModFeatures {
    #[new]
    pub fn new(bc: Bound<PyBytes>) -> PyResult<Self> {
        let bc = bc.as_bytes();
        Self::from_bc(bc)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
}
