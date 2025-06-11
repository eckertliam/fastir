def bitcode_to_ir(bitcode: bytes) -> str: ...

class BBStats:
    name: str
    histogram: dict[str, int]
    opcode_entropy: float
    function_calls: dict[str, int]
    instruction_count: int
    call_count: int
    
class FunctionStats:
    name: str
    basic_block_stats: dict[str, BBStats]

class ModuleStats:
    def __init__(self, bc: bytes) -> None: ...
    function_stats: dict[str, FunctionStats]