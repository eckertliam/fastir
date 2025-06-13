def llvm_inline_pass(bc: bytes) -> bytes: ...

def bc_to_ir(bc: bytes) -> str: ...

class BBFeatures:
    name: str
    histogram: dict[str, int]
    opcode_entropy: float
    function_calls: dict[str, int]
    call_count: int
    instruction_count: int
    
    def mem_access_ratio(self) -> float: ...

class FnFeatures:
    name: str
    bb_feats: dict[str, BBFeatures]
    
class ModFeatures:
    fn_feats: dict[str, FnFeatures]
    def __init__(self, bc: bytes) -> None: ...