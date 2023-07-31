from typing import List
from pyactix.types import DictStrAny


class ConfigError(Exception):
    pass


class AuthenticationError(Exception):
    pass


class ValidationError(Exception):
    """
    This exception raised when operation params do not validate
    Note: this is not the same as pydantic.ValidationError
    the errors attribute as well holds the location of the error(body, form, query, etc.)
    """

    def __init__(self, errors: List[DictStrAny]) -> None:
        super().__init__()
        self.errors = errors
