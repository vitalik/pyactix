from typing import Any, Callable, Dict, TypeVar

__all__ = ["DictStrAny", "TCallable"]

DictStrAny = Dict[str, Any]

TCallable = TypeVar("TCallable", bound=Callable[..., Any])
