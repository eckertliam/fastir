use std::collections::HashMap;

use polars::prelude::*;
use rayon::prelude::*;

/// A candidate for inlining
pub struct InlineCandidate {
    /// the name of the callee function
    pub callee: String,
    /// the name of the caller function
    pub caller: String,
    /// the number of instructions in the callee function
    pub callee_instruction_count: usize,
    /// the number of basic blocks in the callee function
    pub callee_bb_count: usize,
    /// the number of arguments the callee function takes
    pub callee_arg_count: usize,
    /// whether the callee has variable arguments
    pub callee_has_var_args: bool,
    /// whether the callee has `alwaysinline` attribute
    pub callee_has_always_inline: bool,
    /// whether the callee has `noinline` attribute
    pub callee_has_no_inline: bool,
    /// whether the callee has `inlinehint` attribute
    pub callee_has_inline_hint: bool,
    /// whether the callee is recursive
    pub callee_is_recursive: bool,
    /// callee outgoing call count
    pub callee_outgoing_call_count: usize,

    /// caller basic block count
    pub caller_bb_count: usize,
    /// caller instruction count
    pub caller_instruction_count: usize,
    /// whether the caller is recursive
    pub caller_is_recursive: bool,
    /// caller outgoing call count
    pub caller_outgoing_call_count: usize,

    /// caller to callee instruction count ratio
    pub caller_to_call_instr_ratio: f64,
}


/// The analysis result of inlining
pub struct InlineAnalysis {
    /// inline candidates
    pub candidates: Vec<InlineCandidate>,
}

impl InlineAnalysis {
    fn build_column<T: Send>(&self, f: impl Fn(&InlineCandidate) -> T + Send + Sync) -> Vec<T> {
        self.candidates.par_iter().map(f).collect()
    }

    pub fn get_dataframe(&self) -> Result<DataFrame, PolarsError> {
        df!("callee" => self.build_column(|c| c.callee.clone()),
            "caller" => self.build_column(|c| c.caller.clone()),
            "callee_instruction_count" => self.build_column(|c| c.callee_instruction_count as i64),
            "callee_bb_count" => self.build_column(|c| c.callee_bb_count as i64),
            "callee_arg_count" => self.build_column(|c| c.callee_arg_count as i64),
            "callee_has_var_args" => self.build_column(|c| c.callee_has_var_args as i64),
            "callee_has_always_inline" => self.build_column(|c| c.callee_has_always_inline as i64),
            "callee_has_no_inline" => self.build_column(|c| c.callee_has_no_inline as i64),
            "callee_has_inline_hint" => self.build_column(|c| c.callee_has_inline_hint as i64),
            "callee_is_recursive" => self.build_column(|c| c.callee_is_recursive as i64),
            "callee_outgoing_call_count" => self.build_column(|c| c.callee_outgoing_call_count as i64),
            "caller_bb_count" => self.build_column(|c| c.caller_bb_count as i64),
            "caller_instruction_count" => self.build_column(|c| c.caller_instruction_count as i64),
            "caller_is_recursive" => self.build_column(|c| c.caller_is_recursive as i64),
            "caller_outgoing_call_count" => self.build_column(|c| c.caller_outgoing_call_count as i64),
            "caller_to_call_instr_ratio" => self.build_column(|c| c.caller_to_call_instr_ratio as f64),
        )
    }
}