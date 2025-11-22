from typing import List, Optional

from lib.cst_inline import byte_span_to_loc


def byte_offset(text: str, char_index: int) -> int:
    """Convert a character index in a Python string to a UTF-8 byte offset."""
    if char_index <= 0:
        return 0
    # Encoding the prefix handles multi-byte characters correctly.
    return len(text[:char_index].encode("utf-8"))


def line_starts(text: str) -> List[int]:
    """Return UTF-8 byte offsets for the start of each line."""
    starts = [0]
    for idx, b in enumerate(text.encode("utf-8")):
        if b == 0x0A:  # '\n'
            starts.append(idx + 1)
    return starts


def span_to_loc(text: str, start_char: int, end_char: int, starts: Optional[List[int]] = None) -> dict:
    """Convert a character span to a location dict using UTF-8 byte offsets."""
    line_starts_vec = starts or line_starts(text)
    b_start = byte_offset(text, start_char)
    b_end = byte_offset(text, end_char)
    return byte_span_to_loc(b_start, b_end, line_starts_vec)


def point_to_loc(text: str, start_char: int, length: int = 1, starts: Optional[List[int]] = None) -> dict:
    """Convert a single-point character index to a one-token location."""
    return span_to_loc(text, start_char, start_char + length, starts)
