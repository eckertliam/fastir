import io
from typing import Optional
from llvm_tools import extract_inline_features as llvm_extract_inline_features
import polars as pl

def extract_inline_features(bc: bytes) -> Optional[pl.DataFrame]:
    """Extract inline features from LLVM bitcode.
    
    Args:
        bc: LLVM bitcode bytes
        
    Returns:
        DataFrame with inline features or None if extraction fails
    """
    try:
        # llvm_tools.extract_inline_features returns a dataframe serialized to IPC (Arrow) bytes
        df_bytes = llvm_extract_inline_features(bc)
        # Deserialize the IPC-formatted DataFrame
        return pl.read_ipc(io.BytesIO(df_bytes))
    except Exception as e:
        print(f"Error extracting inline features: {e}")
        return None