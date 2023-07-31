from pyactix import Router, PyActixAPI


api = PyActixAPI()

root = Router()
child1 = Router()
child2 = Router()


api.add_router("/api/", root)


@api.get("toplevel", operation_id="toplevel")
def toplevel(request):
    return "Hello World!"


@root.get("/one", operation_id="o1")
def one(request):
    return "Hello World!"


@child1.get("/two", operation_id="o2")
def two(request):
    return "Hello World!"


@child2.get("/three/", operation_id="o3")
def three(request):
    return "Hello World!"


root.add_router("/sub1", child1)
child1.add_router("/sub2", child2)


def test_router():
    result = root.gather_operations("")
    result = [(pth, o.operation_id) for pth, o in result]
    assert result == [
        ('/one', 'o1'),
        ('/sub1/two', 'o2'),
        ('/sub1/sub2/three/', 'o3'),
    ]


def test_api():
    result = api.gather_operations()
    result = [(pth, o.operation_id) for pth, o in result]
    print(result)
    assert result == [
        ('/toplevel', 'toplevel'),
        ('/api/one', 'o1'),
        ('/api/sub1/two', 'o2'),
        ('/api/sub1/sub2/three/', 'o3'),
    ]
