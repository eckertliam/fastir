use std::collections::HashMap;

use inkwell::{basic_block::BasicBlock, values::{FunctionValue, InstructionOpcode}};

use crate::llvm_module::LLVMModule;




fn function_histogram(func: &FunctionValue) -> Result<HashMap<String, usize>, String> {
    let mut histogram = HashMap::with_capacity(func.count_basic_blocks() as usize * 8); // heuristic to avoid rehashing
    for basic_block in func.get_basic_blocks() {
        for instruction in basic_block.get_instructions() {
            let opcode = instruction.get_opcode();
            let opcode_name = opcode_to_string(opcode);
            *histogram.entry(opcode_name).or_insert(0) += 1;
        }
    }
    Ok(histogram)
}

pub fn module_histogram(
    module: &LLVMModule,
) -> Result<HashMap<String, HashMap<String, usize>>, String> {
    module
        .get_functions()
        .map(|func| {
            let name = func
                .get_name()
                .to_str()
                .map_err(|e| format!("Failed to convert function name to string: {:?}", e))?;
            let histogram = function_histogram(&func)?;
            Ok((name.to_string(), histogram))
        })
        .collect()
}
