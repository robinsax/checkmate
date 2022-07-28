from requests import get, post

url = 'http://localhost:8000/v1'

auth = None
def step(meth, path, json=None):
    resp = meth(url + path, json=json, headers={'Authorization': 'Bearer %s'%auth} if auth else None)
    print(resp.json())

    return resp.json()

login = step(post, '/auth', json={'name': 'foo'})
auth = login['token']

new = step(post, '/lobby', json={'game': 'new'})

step(get, '/lobby')

step(get, '/games/' + new['id'])
