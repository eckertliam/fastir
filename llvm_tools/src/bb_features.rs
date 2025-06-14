use std::collections::HashMap;

// TODO: implement a function to calculate how many branches this block can branch to

use llvm_ir::{BasicBlock, Instruction, Operand, Constant};
use pyo3::{pyclass, pymethods, PyResult};

#[pyclass]
#[derive(Clone)]
pub struct BBFeatures {
    _basic_block: BasicBlock,

    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub histogram: HashMap<String, usize>,
    #[pyo3(get)]
    pub opcode_entropy: f64,
    #[pyo3(get)]
    pub function_calls: HashMap<String, usize>,
    #[pyo3(get)]
    pub call_count: usize,
    #[pyo3(get)]
    pub instruction_count: usize,
}

impl BBFeatures {
    pub fn new(basic_block: &BasicBlock) -> Self {
        let name = basic_block.name.to_string();
        let histogram = bb_histogram(basic_block);
        let opcode_entropy = opcode_entropy(&histogram);
        let (function_calls, call_count) = function_calls(basic_block);
        let instruction_count = basic_block.instrs.len();
        Self {
            _basic_block: basic_block.clone(),
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
impl BBFeatures {
    pub fn mem_access_ratio(&self) -> PyResult<f64> {
        let load_count: f64 = *self.histogram.get("load").unwrap_or(&0) as f64;
        let store_count: f64 = *self.histogram.get("store").unwrap_or(&0) as f64;
        Ok((load_count + store_count) / self.instruction_count as f64)
    }
}

fn instruction_to_string(instruction: &Instruction) -> &'static str {
    match instruction {
        // Integer binary ops
        Instruction::Add(_) => "add",
        Instruction::Sub(_) => "sub",
        Instruction::Mul(_) => "mul",
        Instruction::UDiv(_) => "udiv",
        Instruction::SDiv(_) => "sdiv",
        Instruction::URem(_) => "urem",
        Instruction::SRem(_) => "srem",

        // Bitwise binary ops
        Instruction::And(_) => "and",
        Instruction::Or(_) => "or",
        Instruction::Xor(_) => "xor",
        Instruction::Shl(_) => "shl",
        Instruction::LShr(_) => "lshr",
        Instruction::AShr(_) => "ashr",

        // Floating-point ops
        Instruction::FAdd(_) => "fadd",
        Instruction::FSub(_) => "fsub",
        Instruction::FMul(_) => "fmul",
        Instruction::FDiv(_) => "fdiv",
        Instruction::FRem(_) => "frem",
        Instruction::FNeg(_) => "fneg",

        // Vector ops
        Instruction::ExtractElement(_) => "extractelement",
        Instruction::InsertElement(_) => "insertelement",
        Instruction::ShuffleVector(_) => "shufflevector",

        // Aggregate ops
        Instruction::ExtractValue(_) => "extractvalue",
        Instruction::InsertValue(_) => "insertvalue",

        // Memory-related ops
        Instruction::Alloca(_) => "alloca",
        Instruction::Load(_) => "load",
        Instruction::Store(_) => "store",
        Instruction::Fence(_) => "fence",
        Instruction::CmpXchg(_) => "cmpxchg",
        Instruction::AtomicRMW(_) => "atomicrmw",
        Instruction::GetElementPtr(_) => "getelementptr",

        // Conversion ops
        Instruction::Trunc(_) => "trunc",
        Instruction::ZExt(_) => "zext",
        Instruction::SExt(_) => "sext",
        Instruction::FPTrunc(_) => "fptrunc",
        Instruction::FPExt(_) => "fpext",
        Instruction::FPToUI(_) => "fptoui",
        Instruction::FPToSI(_) => "fptosi",
        Instruction::UIToFP(_) => "uitofp",
        Instruction::SIToFP(_) => "sitofp",
        Instruction::PtrToInt(_) => "ptrtoint",
        Instruction::IntToPtr(_) => "inttoptr",
        Instruction::BitCast(_) => "bitcast",
        Instruction::AddrSpaceCast(_) => "addrspacecast",

        // LLVM's "other operations" category
        Instruction::ICmp(_) => "icmp",
        Instruction::FCmp(_) => "fcmp",
        Instruction::Phi(_) => "phi",
        Instruction::Select(_) => "select",
        Instruction::Call(_) => "call",
        Instruction::VAArg(_) => "vaarg",
        Instruction::LandingPad(_) => "landingpad",
        Instruction::CatchPad(_) => "catchpad",
        Instruction::CleanupPad(_) => "cleanuppad",
        Instruction::Freeze(_) => "freeze",
    }
}

fn bb_histogram(bb: &BasicBlock) -> HashMap<String, usize> {
    let mut histogram = HashMap::new();
    for instr in bb.instrs.iter() {
        let instr_str = instruction_to_string(instr).to_string();
        *histogram.entry(instr_str).or_insert(0) += 1;
    }
    histogram
}

// shannon entropy of opcode distribution
// H = -sum(p_i * log2(p_i))
fn opcode_entropy(histogram: &HashMap<String, usize>) -> f64 {
    let total = histogram.values().sum::<usize>() as f64;
    let inv_total = 1.0 / total; // Avoid repeated division

    histogram
        .values()
        .map(|&count| {
            let p = count as f64 * inv_total;
            -p * p.log2()
        })
        .sum()
}

fn function_calls(bb: &BasicBlock) -> (HashMap<String, usize>, usize) {
    let mut calls = HashMap::new();
    let mut call_count = 0;
    for instr in bb.instrs.iter() {
        match instr {
            Instruction::Call(call) => {
                call_count += 1;
                if let Some(operand) = call.function.as_ref().right() {
                    let function_name = match operand {
                        // Direct function call via global reference
                        Operand::ConstantOperand(constant_ref) => {
                            match constant_ref.as_ref() {
                                llvm_ir::Constant::GlobalReference { name, .. } => {
                                    Some(name.to_string())
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    };
                    
                    if let Some(name) = function_name {
                        *calls.entry(name).or_insert(0) += 1;
                    }
                }
            }
            _ => {}
        }
    }
    (calls, call_count)
}
