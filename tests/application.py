import asyncio
from pyactix import PyActixAPI, Server, SocketHeld

api = PyActixAPI()


def _serialize_request(request):
    import json

    data = {}
    for attr in ['method', 'scheme', 'host', 'path', 'path_params', 'query_string', 'query_params', 'headers']:
        data[attr] = getattr(request, attr)
    return json.dumps(data)


@api.get("/request_details")
def request_details(request, *a, **kw):
    return _serialize_request(request)


@api.get("/request_details/{id}/{name}")
def request_details_params(request, *a, **kw):
    return _serialize_request(request)


@api.get("/async")
async def callback1(request, *a, **kw):
    await asyncio.sleep(0.1)
    return str(('Hello World! async', a, kw))


@api.get("/sync/{id}")
def callback2(request, *a, **kw):
    return str(('Hello World! NOT ASYNC', a, kw))


@api.post("/sync/{id}")
def with_error(request, *a, **kw):
    raise Exception('Error')


server = Server(api.get_server_config())
socket = SocketHeld('127.0.0.1', 8080)
server.start(socket, 6)
