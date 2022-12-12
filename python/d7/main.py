from dataclasses import dataclass
from typing import Generic, TypeVar

T = TypeVar('T')

class Node(Generic[T]):
    def __init__(self) -> None:
        self.