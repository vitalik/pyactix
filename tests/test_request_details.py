import pytest


def test_request_all(client):
    response = client.get('/request_details')
    assert response.status_code == 200, response.content
    data = response.json()
    print(data)
    del data['headers']['user-agent']  # avoiding version check
    assert data == {
        'method': 'GET',
        'scheme': 'http',
        'host': '127.0.0.1:8080',
        'path': '/request_details',
        'path_params': {},
        'query_string': '',
        'query_params': [],
        'headers': {
            'host': '127.0.0.1:8080',
            'connection': 'keep-alive',
            'accept': '*/*',
            'accept-encoding': 'gzip, deflate',
        },
    }


def test_request_404(client):
    response = client.get('/request_details/1')
    assert response.status_code == 404


@pytest.mark.parametrize(
    "path_suffix,req_params,expected",
    [
        ('', dict(), dict(query_string='')),
        ('', dict(params={'x': 1}), dict(query_string='x=1', query_params=[['x', '1']])),
        ('', dict(params={'y': 1}), dict(query_string='y=1', query_params=[['y', '1']])),
        ('', dict(params={'y': [1, 2, 3]}), dict(query_string='y=1&y=2&y=3')),
        ('', dict(headers={'X-Test': 'value'}), dict(headers={'x-test': 'value'})),
        ('/1/test', {}, dict(path_params={'id': '1', 'name': 'test'})),
    ],
)
def test_batch(client, path_suffix, req_params, expected):
    print(req_params, expected)
    response = client.get('/request_details' + path_suffix, **req_params)
    assert response.status_code == 200, response.content
    resp_data = response.json()
    print(resp_data)
    for k, v in expected.items():
        if k == 'headers':
            for h_k, h_val in v.items():
                assert resp_data[k][h_k] == h_val
        else:
            assert resp_data[k] == v, f'k={k}, v={v}, resp_data={resp_data[k]}'
