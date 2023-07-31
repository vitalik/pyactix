import asyncio
from pyactix import get_version, PyActixAPI, Server, SocketHeld

print('Version', get_version())


api = PyActixAPI()


@api.get("/async")
async def callback1(request, *a, **kw):
    await asyncio.sleep(0.1)
    return str(('Hello World! async', a, kw))


@api.get("/sync/{id}/{name}")
def callback2(request, *a, **kw):
    result = []
    for i in dir(request):
        if not i.startswith('_'):
            value = getattr(request, i)
            if callable(value):
                value = value()
            result.append(f'{i}: {value}')
    return '\n'.join(result)


@api.post("/sync/{id}")
def with_error(request, *a, **kw):
    raise Exception('Error')


server = Server(api.get_server_config())
socket = SocketHeld('127.0.0.1', 8000)
server.start(socket, 6)
