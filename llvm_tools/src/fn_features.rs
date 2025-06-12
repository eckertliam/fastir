use std::collections::HashMap;

use llvm_ir::Function;
use pyo3::pyclass;
use rayon::prelude::*;

use crate::bb_features::BBFeatures;

#[pyclass]
#[derive(Clone)]
pub struct FnFeatures {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub bb_feats: HashMap<String, BBFeatures>,
}

impl FnFeatures {
    pub fn new(function: &Function) -> Self {
        let name = function.name.to_string();
        let bb_feats = function
            .basic_blocks
            .par_iter()
            .map(|bb| {
                let bb_feat = BBFeatures::new(bb);
                (bb_feat.name.clone(), bb_feat)
            })
            .collect();
        Self { name, bb_feats }
    }
}
