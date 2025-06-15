use std::collections::{HashMap, HashSet};

use llvm_ir::{function::{FunctionAttribute, FunctionDeclaration}, Function};
use pyo3::pyclass;
use rayon::prelude::*;

use crate::bb_features::BBFeatures;

#[pyclass]
#[derive(Clone)]
/// Features of a function
pub struct FnFeatures {
    #[pyo3(get)]
    /// (bb_name, callee_name)
    pub calls: HashSet<(String, String)>,
    #[pyo3(get)]
    /// The name of the function
    pub name: String,
    #[pyo3(get)]
    /// The basic blocks in the function
    pub bb_feats: HashMap<String, BBFeatures>,
    #[pyo3(get)]
    /// The number of arguments the function takes
    pub arg_count: u64,
    #[pyo3(get)]
    /// The number of instructions in the function
    pub instruction_count: u64,
    #[pyo3(get)]
    /// Whether the function has var args
    pub has_var_args: bool,
    #[pyo3(get)]
    /// The number of basic blocks in the function
    pub bb_count: u64,
    #[pyo3(get)]
    /// Whether the function has the `alwaysinline` attribute
    pub has_always_inline: bool,
    #[pyo3(get)]
    /// Whether the function has the `noinline` attribute
    pub has_no_inline: bool,
    #[pyo3(get)]
    /// Whether the function calls itself
    pub is_recursive: bool,
    #[pyo3(get)]
    /// The number of outgoing calls from the function
    pub outgoing_call_count: u64,
}

impl FnFeatures {
    pub fn from_def(function: &Function) -> Self {
        let name = function.name.to_string();
        let bb_feats: HashMap<String, BBFeatures> = function
            .basic_blocks
            .par_iter()
            .map(|bb| {
                let bb_feat = BBFeatures::new(bb);
                (bb_feat.name.clone(), bb_feat)
            })
            .collect();

        let calls = bb_feats
            .values()
            .flat_map(|bb| {
                bb.function_calls
                    .iter()
                    .map(|(callee_name, _)| (bb.name.clone(), callee_name.clone()))
            })
            .collect();
        let arg_count = function.parameters.len() as u64;
        let instruction_count = bb_feats
            .values()
            .map(|bb| bb.instruction_count as u64)
            .sum::<u64>();
        let has_var_args = function.is_var_arg;
        let bb_count = bb_feats.len() as u64;
        let has_always_inline = function
            .function_attributes
            .contains(&FunctionAttribute::AlwaysInline);
        let has_no_inline = function
            .function_attributes
            .contains(&FunctionAttribute::NoInline);
        let is_recursive = bb_feats
            .values()
            .any(|bb| bb.function_calls.contains_key(&name));
        let outgoing_call_count = bb_feats
            .values()
            .map(|bb| bb.call_count as u64)
            .sum::<u64>();
        Self {
            calls,
            name,
            bb_feats,
            arg_count,
            instruction_count,
            has_var_args,
            bb_count,
            has_always_inline,
            has_no_inline,
            is_recursive,
            outgoing_call_count,
        }
    }

    pub fn from_declaration(decl: &FunctionDeclaration) -> Self {
        let name = decl.name.to_string();
        let arg_count = decl.parameters.len() as u64;
        let has_var_args = decl.is_var_arg;
        Self {
            calls: HashSet::new(),
            name,
            bb_feats: HashMap::new(),
            arg_count,
            instruction_count: 0,
            has_var_args,
            bb_count: 0,
            has_always_inline: false,
            has_no_inline: false,
            is_recursive: false,
            outgoing_call_count: 0,
        }
    }
}
