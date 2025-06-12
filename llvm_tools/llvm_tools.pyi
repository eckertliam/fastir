class BBFeatures:
    name: str
    histogram: dict[str, int]
    opcode_entropy: float
    function_calls: dict[str, int]
    call_count: int
    instruction_count: int

class FnFeatures:
    name: str
    bb_feats: dict[str, BBFeatures]
    
class ModFeatures:
    fn_feats: dict[str, FnFeatures]
    def __init__(self, bc: bytes) -> None: ...