import inspect
from typing import Callable


def normalize_path(path: str) -> str:
    while "//" in path:
        path = path.replace("//", "/")
    return path


def is_async(f: Callable) -> bool:
    return inspect.iscoroutinefunction(f) or inspect.iscoroutinefunction(
        getattr(f, "__call__", None)
    )
