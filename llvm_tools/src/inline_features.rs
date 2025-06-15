use std::collections::HashSet;

use polars::io::ipc::IpcWriter;
use polars::prelude::*;
use pyo3::{prelude::*, types::PyBytes, Bound, PyResult};

use crate::mod_features::ModFeatures;

/*
Inline Features per callsite:
- callee_name: String
- callee_instruction_count: u64
- callee_bb_count: u64
- callee_arg_count: u64
- callee_has_var_args: bool
- callee_has_always_inline: bool
- callee_has_no_inline: bool
- callee_is_recursive: bool
- callee_outgoing_call_count: u64

- caller_name: String
- caller_bb_count: u64
- caller_instruction_count: u64
- caller_is_recursive: bool
- caller_outgoing_call_count: u64

- caller_to_callee_instr_ratio: f64
- bb_name: String
- llvm_inlining_decision: bool
*/

#[pyfunction]
pub fn extract_inline_features(bc: Bound<PyBytes>) -> PyResult<Vec<u8>> {
    let mod_features = ModFeatures::new(bc.clone())?;
    let inlined_mod_features = ModFeatures::inlined_mod_features(bc.as_bytes())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    // (caller_name, bb_name, callee_name)
    let callsite_intersections: HashSet<&(String, String, String)> = mod_features
        .call_sites
        .intersection(&inlined_mod_features.call_sites)
        .collect();

    // define a vec for each column
    let mut callee_name_vec: Vec<String> = vec![];
    let mut callee_instruction_count_vec: Vec<u64> = vec![];
    let mut callee_bb_count_vec: Vec<u64> = vec![];
    let mut callee_arg_count_vec: Vec<u64> = vec![];
    let mut callee_has_var_args_vec: Vec<bool> = vec![];
    let mut callee_has_always_inline_vec: Vec<bool> = vec![];
    let mut callee_has_no_inline_vec: Vec<bool> = vec![];
    let mut callee_is_recursive_vec: Vec<bool> = vec![];
    let mut callee_outgoing_call_count_vec: Vec<u64> = vec![];

    let mut caller_name_vec: Vec<String> = vec![];
    let mut caller_bb_count_vec: Vec<u64> = vec![];
    let mut caller_instruction_count_vec: Vec<u64> = vec![];
    let mut caller_is_recursive_vec: Vec<bool> = vec![];
    let mut caller_outgoing_call_count_vec: Vec<u64> = vec![];

    let mut caller_to_callee_instr_ratio_vec: Vec<f64> = vec![];
    let mut bb_name_vec: Vec<String> = vec![];
    let mut llvm_inlining_decision_vec: Vec<bool> = vec![];

    for (caller_name, bb_name, callee_name) in mod_features.call_sites.clone() {
        let (callee_features, caller_features) = match (mod_features.fn_feats.get(&callee_name), mod_features.fn_feats.get(&caller_name)) {
            (Some(callee_features), Some(caller_features)) => (callee_features, caller_features),
            (None, Some(_)) => {
                // Callee is external - this is expected for intrinsics, library calls, etc.
                continue;
            },
            (Some(_), None) => {
                // Caller not found - this shouldn't happen if call site was extracted from fn_feats
                eprintln!("Warning: Caller function {} not found", caller_name);
                continue;
            },
            (None, None) => {
                eprintln!("Warning: Both caller {} and callee {} not found", caller_name, callee_name);
                continue;
            }
        };

        callee_name_vec.push(callee_name.clone());
        callee_instruction_count_vec.push(callee_features.instruction_count);
        callee_bb_count_vec.push(callee_features.bb_count);
        callee_arg_count_vec.push(callee_features.arg_count);
        callee_has_var_args_vec.push(callee_features.has_var_args);
        callee_has_always_inline_vec.push(callee_features.has_always_inline);
        callee_has_no_inline_vec.push(callee_features.has_no_inline);
        callee_is_recursive_vec.push(callee_features.is_recursive);
        callee_outgoing_call_count_vec.push(callee_features.outgoing_call_count);

        caller_name_vec.push(caller_name.clone());
        caller_bb_count_vec.push(caller_features.bb_count);
        caller_instruction_count_vec.push(caller_features.instruction_count);
        caller_is_recursive_vec.push(caller_features.is_recursive);
        caller_outgoing_call_count_vec.push(caller_features.outgoing_call_count);

        caller_to_callee_instr_ratio_vec.push(
            caller_features.instruction_count as f64 / callee_features.instruction_count as f64,
        );
        bb_name_vec.push(bb_name.clone());
        llvm_inlining_decision_vec.push(callsite_intersections.contains(&(
            caller_name,
            bb_name,
            callee_name,
        )));
    }

    let mut df = df!(
        "callee_name" => callee_name_vec,
        "callee_instruction_count" => callee_instruction_count_vec,
        "callee_bb_count" => callee_bb_count_vec,
        "callee_arg_count" => callee_arg_count_vec,
        "callee_has_var_args" => callee_has_var_args_vec,
        "callee_has_always_inline" => callee_has_always_inline_vec,
        "callee_has_no_inline" => callee_has_no_inline_vec,
        "callee_is_recursive" => callee_is_recursive_vec,
        "callee_outgoing_call_count" => callee_outgoing_call_count_vec,
        "caller_name" => caller_name_vec,
        "caller_bb_count" => caller_bb_count_vec,
        "caller_instruction_count" => caller_instruction_count_vec,
        "caller_is_recursive" => caller_is_recursive_vec,
        "caller_outgoing_call_count" => caller_outgoing_call_count_vec,
        "caller_to_callee_instr_ratio" => caller_to_callee_instr_ratio_vec,
        "bb_name" => bb_name_vec,
        "llvm_inlining_decision" => llvm_inlining_decision_vec,
    )
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Serialize to IPC format for cross-language compatibility
    let mut buf = Vec::new();
    IpcWriter::new(&mut buf)
        .finish(&mut df)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    Ok(buf)
}
