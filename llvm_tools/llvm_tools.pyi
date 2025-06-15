def llvm_inline_pass(bc: bytes) -> bytes: ...
def bc_to_ir(bc: bytes) -> str: ...
def extract_inline_features(bc: bytes) -> bytes: ...

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
    arg_count: int
    instruction_count: int
    has_var_args: bool
    has_inline_hint: bool
    bb_count: int
    has_always_inline: bool
    has_no_inline: bool
    is_recursive: bool
    outgoing_call_count: int
    calls: set[tuple[str, str]]

class ModFeatures:
    fn_feats: dict[str, FnFeatures]
    call_sites: set[tuple[str, str, str]]
    
    def __init__(self, bc: bytes) -> None: ...
