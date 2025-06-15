use llvm_sys::{
    analysis::{LLVMVerifierFailureAction, LLVMVerifyModule},
    bit_reader::LLVMParseBitcodeInContext2,
    bit_writer::LLVMWriteBitcodeToMemoryBuffer,
    core::*,
    error::{LLVMDisposeErrorMessage, LLVMGetErrorMessage},
    target::*,
    target_machine::*,
    transforms::pass_builder::{
        LLVMCreatePassBuilderOptions, LLVMDisposePassBuilderOptions, LLVMRunPasses,
    },
};
use std::{ffi::CString, ptr};

pub fn run_inline_pass(bc: &[u8]) -> Result<Vec<u8>, String> {
    unsafe {
        // fresh llvm context & load the bitcode
        let ctx = LLVMContextCreate();
        let buf_name = CString::new("input_bc").unwrap();
        let mbuf = LLVMCreateMemoryBufferWithMemoryRangeCopy(
            bc.as_ptr() as *const i8,
            bc.len(),
            buf_name.as_ptr(),
        );
        let mut module = ptr::null_mut();
        if LLVMParseBitcodeInContext2(ctx, mbuf, &mut module) != 0 {
            return Err("could not parse bitcode".into());
        }

        // sanity-check that the module is valid
        if LLVMVerifyModule(
            module,
            LLVMVerifierFailureAction::LLVMPrintMessageAction,
            ptr::null_mut(),
        ) != 0
        {
            return Err("input module failed verification".into());
        }

        // build & run only the inliner with the PassBuilder C-API
        let pipeline = CString::new("cgscc(inline)").unwrap();

        // use null target machine for target-independent inlining pass
        let pb_opts = LLVMCreatePassBuilderOptions();

        let error_ref = LLVMRunPasses(module, pipeline.as_ptr(), ptr::null_mut(), pb_opts);
        if !error_ref.is_null() {
            // get error message from the error reference
            let error_msg = LLVMGetErrorMessage(error_ref);
            let error_str = if !error_msg.is_null() {
                let error_str = std::ffi::CStr::from_ptr(error_msg)
                    .to_string_lossy()
                    .into_owned();
                LLVMDisposeErrorMessage(error_msg);
                error_str
            } else {
                "LLVMRunPasses failed with unknown error".to_string()
            };
            return Err(error_str);
        }

        // re-serialize as bitcode
        let bc_buf = LLVMWriteBitcodeToMemoryBuffer(module);
        let start = LLVMGetBufferStart(bc_buf) as *const u8;
        let len = LLVMGetBufferSize(bc_buf);
        let ret = std::slice::from_raw_parts(start, len).to_vec();

        // clean up
        LLVMDisposePassBuilderOptions(pb_opts);
        LLVMDisposeMemoryBuffer(mbuf);
        LLVMDisposeModule(module);
        LLVMContextDispose(ctx);

        Ok(ret)
    }
}

pub fn bitcode_to_ir(bc: &[u8]) -> Result<String, String> {
    unsafe {
        // create fresh llvm context & load the bitcode
        let ctx = LLVMContextCreate();
        let buf_name = CString::new("input_bc").unwrap();
        let mbuf = LLVMCreateMemoryBufferWithMemoryRangeCopy(
            bc.as_ptr() as *const i8,
            bc.len(),
            buf_name.as_ptr(),
        );

        let mut module = ptr::null_mut();
        if LLVMParseBitcodeInContext2(ctx, mbuf, &mut module) != 0 {
            LLVMDisposeMemoryBuffer(mbuf);
            LLVMContextDispose(ctx);
            return Err("could not parse bitcode".into());
        }

        // sanity check that the module is valid
        if LLVMVerifyModule(
            module,
            LLVMVerifierFailureAction::LLVMPrintMessageAction,
            ptr::null_mut(),
        ) != 0
        {
            LLVMDisposeModule(module);
            LLVMDisposeMemoryBuffer(mbuf);
            LLVMContextDispose(ctx);
            return Err("input module failed verification".into());
        }

        // convert module to LLVM IR string
        let ir_cstr = LLVMPrintModuleToString(module);
        if ir_cstr.is_null() {
            LLVMDisposeModule(module);
            LLVMDisposeMemoryBuffer(mbuf);
            LLVMContextDispose(ctx);
            return Err("failed to convert module to string".into());
        }

        let ir_string = std::ffi::CStr::from_ptr(ir_cstr)
            .to_string_lossy()
            .into_owned();

        // clean up
        LLVMDisposeMessage(ir_cstr);
        LLVMDisposeModule(module);
        LLVMDisposeMemoryBuffer(mbuf);
        LLVMContextDispose(ctx);

        Ok(ir_string)
    }
}
