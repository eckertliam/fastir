from datasets import load_dataset

class DataStream:
    def __init__(self) -> None:
        self._ds = iter(load_dataset('llvm-ml/ComPile', split='train', streaming=True))
        
    def __iter__(self):
        return self
    
    def __next__(self) -> bytes:
        return next(self._ds)['content']

