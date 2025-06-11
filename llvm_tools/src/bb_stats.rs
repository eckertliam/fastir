use std::collections::HashMap;

// TODO: implement a function to calculate the ratio of load/store instructions against all others
// TODO: implement a function to calculate how many branches this block can branch to

use inkwell::{
    basic_block::BasicBlock,
    values::{CallSiteValue, InstructionOpcode},
};
use pyo3::{pyclass, pymethods};
use rayon::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct BBStats {
    pub name: String,
    pub histogram: HashMap<String, usize>,
    pub opcode_entropy: f64,
    pub function_calls: HashMap<String, usize>,
    pub call_count: usize,
    pub instruction_count: usize,
}

impl BBStats {
    pub fn new(basic_block: &BasicBlock) -> Self {
        let histogram = bb_histogram(basic_block);
        let opcode_entropy = opcode_entropy(&histogram);
        let function_calls = function_calls(basic_block);
        let name = basic_block.get_name().to_string_lossy().to_string();
        let call_count = function_calls.values().sum();
        let instruction_count = histogram.values().sum();
        Self {
            name,
            histogram,
            opcode_entropy,
            function_calls,
            call_count,
            instruction_count,
        }
    }
}

#[pymethods]
impl BBStats {
    #[getter]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[getter]
    pub fn histogram(&self) -> &HashMap<String, usize> {
        &self.histogram
    }

    #[getter]
    pub fn opcode_entropy(&self) -> f64 {
        self.opcode_entropy
    }

    #[getter]
    pub fn function_calls(&self) -> &HashMap<String, usize> {
        &self.function_calls
    }

    #[getter]
    pub fn instruction_count(&self) -> usize {
        self.instruction_count
    }

    #[getter]
    pub fn call_count(&self) -> usize {
        self.call_count
    }
}

fn opcode_to_string(opcode: InstructionOpcode) -> String {
    match opcode {
        InstructionOpcode::Add => "add".to_string(),
        InstructionOpcode::AddrSpaceCast => "addrspacecast".to_string(),
        InstructionOpcode::Alloca => "alloca".to_string(),
        InstructionOpcode::And => "and".to_string(),
        InstructionOpcode::AtomicCmpXchg => "atomic_cmpxchg".to_string(),
        InstructionOpcode::AtomicRMW => "atomic_rmw".to_string(),
        InstructionOpcode::BitCast => "bitcast".to_string(),
        InstructionOpcode::Br => "br".to_string(),
        InstructionOpcode::Call => "call".to_string(),
        InstructionOpcode::CallBr => "callbr".to_string(),
        InstructionOpcode::CatchPad => "catchpad".to_string(),
        InstructionOpcode::CatchRet => "catchret".to_string(),
        InstructionOpcode::CatchSwitch => "catchswitch".to_string(),
        InstructionOpcode::CleanupPad => "cleanuppad".to_string(),
        InstructionOpcode::CleanupRet => "cleanupret".to_string(),
        InstructionOpcode::ExtractElement => "extractelement".to_string(),
        InstructionOpcode::ExtractValue => "extractvalue".to_string(),
        InstructionOpcode::FNeg => "fneg".to_string(),
        InstructionOpcode::FAdd => "fadd".to_string(),
        InstructionOpcode::FCmp => "fcmp".to_string(),
        InstructionOpcode::FDiv => "fdiv".to_string(),
        InstructionOpcode::Fence => "fence".to_string(),
        InstructionOpcode::FMul => "fmul".to_string(),
        InstructionOpcode::FPExt => "fpext".to_string(),
        InstructionOpcode::FPToSI => "fptosi".to_string(),
        InstructionOpcode::FPToUI => "fptoui".to_string(),
        InstructionOpcode::FPTrunc => "fptrunc".to_string(),
        InstructionOpcode::Freeze => "freeze".to_string(),
        InstructionOpcode::FRem => "frem".to_string(),
        InstructionOpcode::FSub => "fsub".to_string(),
        InstructionOpcode::GetElementPtr => "getelementptr".to_string(),
        InstructionOpcode::ICmp => "icmp".to_string(),
        InstructionOpcode::IndirectBr => "indirectbr".to_string(),
        InstructionOpcode::InsertElement => "insertelement".to_string(),
        InstructionOpcode::InsertValue => "insertvalue".to_string(),
        InstructionOpcode::IntToPtr => "inttoptr".to_string(),
        InstructionOpcode::Invoke => "invoke".to_string(),
        InstructionOpcode::LandingPad => "landingpad".to_string(),
        InstructionOpcode::Load => "load".to_string(),
        InstructionOpcode::LShr => "lshr".to_string(),
        InstructionOpcode::Mul => "mul".to_string(),
        InstructionOpcode::Or => "or".to_string(),
        InstructionOpcode::Phi => "phi".to_string(),
        InstructionOpcode::PtrToInt => "ptrtoint".to_string(),
        InstructionOpcode::Resume => "resume".to_string(),
        InstructionOpcode::Return => "ret".to_string(),
        InstructionOpcode::SDiv => "sdiv".to_string(),
        InstructionOpcode::Select => "select".to_string(),
        InstructionOpcode::SExt => "sext".to_string(),
        InstructionOpcode::Shl => "shl".to_string(),
        InstructionOpcode::ShuffleVector => "shufflevector".to_string(),
        InstructionOpcode::SIToFP => "sitofp".to_string(),
        InstructionOpcode::SRem => "srem".to_string(),
        InstructionOpcode::Store => "store".to_string(),
        InstructionOpcode::Sub => "sub".to_string(),
        InstructionOpcode::Switch => "switch".to_string(),
        InstructionOpcode::Trunc => "trunc".to_string(),
        InstructionOpcode::UDiv => "udiv".to_string(),
        InstructionOpcode::UIToFP => "uitofp".to_string(),
        InstructionOpcode::Unreachable => "unreachable".to_string(),
        InstructionOpcode::URem => "urem".to_string(),
        InstructionOpcode::Xor => "xor".to_string(),
        InstructionOpcode::ZExt => "zext".to_string(),
        _ => unimplemented!("Opcode {:?} not implemented", opcode),
    }
}

fn bb_histogram(bb: &BasicBlock) -> HashMap<String, usize> {
    let mut histogram = HashMap::new();
    for instruction in bb.get_instructions() {
        let opcode = instruction.get_opcode();
        let opcode_name = opcode_to_string(opcode);
        *histogram.entry(opcode_name).or_insert(0) += 1;
    }
    histogram
}

// shannon entropy of opcode distribution
// H = -sum(p_i * log2(p_i))
fn opcode_entropy(histogram: &HashMap<String, usize>) -> f64 {
    let total = histogram.par_iter().map(|(_, count)| *count).sum::<usize>();

    // sum of -p * log2(p) for each opcode
    // we do this in parallel and sum the results
    histogram
        .par_iter()
        .map(|(_, count)| {
            let p = *count as f64 / total as f64;
            -p * p.log2()
        })
        .sum()
}

// TODO: this is very brittle and hacky will probably need to be rewritten
fn function_calls(bb: &BasicBlock) -> HashMap<String, usize> {
    let mut calls = HashMap::new();
    for instruction in bb.get_instructions() {
        match instruction.get_opcode() {
            InstructionOpcode::Call | InstructionOpcode::Invoke => {
                let callsite =
                    CallSiteValue::try_from(instruction).expect("Failed to get callsite");
                let called_func = callsite
                    .get_called_fn_value()
                    .expect("Failed to get called function");
                let called_func_name = called_func.get_name().to_string_lossy().to_string();
                *calls.entry(called_func_name).or_insert(0) += 1;
            }
            _ => {}
        }
    }
    calls
}
