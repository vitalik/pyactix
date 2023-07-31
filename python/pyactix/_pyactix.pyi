from typing import Callable, Union, Any

def get_version(self) -> str: ...

class OperationInfo:
    handler: Callable
    is_async: bool
    method: str
    path: str

    def __init__(
        self, method: str, path: str, handler: Callable, is_async: bool
    ) -> None:
        pass

class Request:
    headers: Any
    host: str
    method: str
    path: str
    path_params: Any
    query_params: Any
    query_string: str
    scheme: str

class Response:
    content: Union[str, bytes]
    headers: dict
    status_code: int
    def __init__(self, content: Union[str, bytes]): ...

class SocketHeld:
    def __init__(self, host: str, port: int): ...

class Server:
    def __init__(self, operations: list[OperationInfo]): ...
    def start(self, socket: SocketHeld): ...
