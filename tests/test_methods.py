import time


def test_hello_world(client):
    response = client.get('/sync/1')
    assert response.status_code == 200, response.content
    assert response.text == "('Hello World! NOT ASYNC', (), {})"


def test_hello_world2(client):
    response = client.get('/async')
    assert response.status_code == 200, response.content
    assert response.text == "('Hello World! async', (), {})"


def test_hello_world3(client):
    response = client.post('/sync/1')
    assert response.status_code == 500, response.content
    assert 'Exception: Error' in response.text
