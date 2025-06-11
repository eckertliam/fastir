use inkwell::{
    context::Context,
    memory_buffer::MemoryBuffer,
    module::{FunctionIterator, Module},
};

pub struct LLVMModule<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
}

impl<'ctx> LLVMModule<'ctx> {
    pub fn from_bitcode(bc: &[u8]) -> Result<Self, String> {
        let context = Box::leak(Box::new(Context::create())); // intentinally leak the context
        let buffer = MemoryBuffer::create_from_memory_range(bc, "module.bc");
        let module =
            Module::parse_bitcode_from_buffer(&buffer, &*context).map_err(|e| e.to_string())?;

        Ok(Self { context, module })
    }

    pub fn to_ir(&self) -> Result<String, String> {
        let llvm_ir = self.module.print_to_string();
        match llvm_ir.to_str() {
            Ok(ir) => Ok(ir.to_string()),
            Err(_) => Err("Failed to convert LLVM IR to valid UTF-8 string".to_string()),
        }
    }

    pub fn get_functions(&self) -> FunctionIterator<'ctx> {
        self.module.get_functions()
    }
}
