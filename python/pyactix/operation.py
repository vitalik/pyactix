from functools import wraps
from typing import (
    Any,
    Callable,
    Dict,
    List,
    Optional,
    Sequence,
    Union,
    cast,
)


from pyactix import Request, Response
from pyactix.utils import is_async
from pyactix.constants import NOT_SET

# from ninja.params_models import TModels
# from ninja.signature import ViewSignature


__all__ = ["Operation"]


class Operation:
    def __init__(
        self,
        path: str,
        methods: List[str],
        view_func: Callable,
        *,
        auth: Optional[Union[Sequence[Callable], Callable, object]] = NOT_SET,
        response: Any = NOT_SET,
        operation_id: Optional[str] = None,
        summary: Optional[str] = None,
        description: Optional[str] = None,
        tags: Optional[List[str]] = None,
        deprecated: Optional[bool] = None,
        by_alias: bool = False,
        exclude_unset: bool = False,
        exclude_defaults: bool = False,
        exclude_none: bool = False,
        include_in_schema: bool = True,
        openapi_extra: Optional[Dict[str, Any]] = None,
    ) -> None:
        self.is_async = False
        self.path: str = path
        self.methods: List[str] = methods
        self.view_func: Callable = view_func
        self.is_async = is_async(self.view_func)
        self.api: "PyActixAPI" = cast("PyActixAPI", None)

        self.auth_param: Optional[Union[Sequence[Callable], Callable, object]] = auth
        self.auth_callbacks: Sequence[Callable] = []
        # self._set_auth(auth)

        # self.signature = ViewSignature(self.path, self.view_func)
        # self.models: TModels = self.signature.models

        # self.response_models: Dict[Any, Any]
        # if response is NOT_SET:
        #     self.response_models = {200: NOT_SET}
        # elif isinstance(response, dict):
        #     self.response_models = self._create_response_model_multiple(response)
        # else:
        #     self.response_models = {200: self._create_response_model(response)}

        self.operation_id = operation_id
        self.summary = summary or self.view_func.__name__.title().replace("_", " ")
        # self.description = description or self.signature.docstring
        self.tags = tags
        self.deprecated = deprecated
        self.include_in_schema = include_in_schema
        self.openapi_extra = openapi_extra

        # Exporting models params
        self.by_alias = by_alias
        self.exclude_unset = exclude_unset
        self.exclude_defaults = exclude_defaults
        self.exclude_none = exclude_none

        if hasattr(view_func, "_pyactix_contribute_to_operation"):
            # Allow 3rd party code to contribute to the operation behaviour
            callbacks: List[Callable] = view_func._pyactix_contribute_to_operation  # type: ignore
            for callback in callbacks:
                callback(self)

    def set_api_instance(self, api: "PyActixAPI", router: "Router") -> None:
        self.api = api
        if self.tags is None:
            if router.tags is not None:
                self.tags = router.tags
        # if self.auth_param == NOT_SET:
        #     if api.auth != NOT_SET:
        #         self._set_auth(self.api.auth)
        #     if router.auth != NOT_SET:
        #         self._set_auth(router.auth)

    def get_callback(self) -> Callable:
        if self.is_async:

            @wraps(self.view_func)
            async def callback(request: Request) -> Any:
                # prepare args
                output = await self.view_func(request)
                # prepare response
                return Response(output)

        else:

            @wraps(self.view_func)
            def callback(request: Request) -> Any:
                # prepare args
                output = self.view_func(request)
                # prepare response
                return Response(output)

        return callback

    # def run(self, request: HttpRequest, **kw: Any):
    #     error = self._run_checks(request)
    #     if error:
    #         return error
    #     try:
    #         temporal_response = self.api.create_temporal_response(request)
    #         values = self._get_values(request, kw, temporal_response)
    #         result = self.view_func(request, **values)
    #         return self._result_to_response(request, result, temporal_response)
    #     except Exception as e:
    #         if isinstance(e, TypeError) and "required positional argument" in str(e):
    #             msg = "Did you fail to use functools.wraps() in a decorator?"
    #             msg = f"{e.args[0]}: {msg}" if e.args else msg
    #             e.args = (msg,) + e.args[1:]
    #         return self.api.on_exception(request, e)
