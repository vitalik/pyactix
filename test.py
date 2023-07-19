import asyncio
import pyactix

print('Version', pyactix.get_version())


async def callback1(*a, **kw):
    await asyncio.sleep(0.1)
    return str(('Hello World! async', a, kw))


def callback2(*a, **kw):
    return str(('Hello World! NOT ASYNC', a, kw))


info1 = pyactix.OperationInfo("GET", "/foo", callback1, is_async=True)
info2 = pyactix.OperationInfo("GET", "/foo/:id", callback2, is_async=False)


server = pyactix.Server([info1, info2])

socket = pyactix.SocketHeld('127.0.0.1', 8000)
server.start(socket, 8)
