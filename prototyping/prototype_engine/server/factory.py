import logging

from falcon import App, CORSMiddleware
from threading import Thread, Event
from werkzeug.serving import BaseWSGIServer, make_server

from .bases import IServer
from .resources import GameResource, PlayersResource
from .parameters import ServerParameters

class ServerStop(BaseException):
    pass

class StoppableServer(BaseWSGIServer):
    _stoppable_signal: Event

    def service_actions(self) -> None:
        if self._stoppable_signal.is_set():
            raise ServerStop()

class SimpleServer(IServer):
    _server: StoppableServer

    def __init__(self, server: StoppableServer) -> None:
        super().__init__()

        self._server = server
        self._server._stoppable_signal = Event()

    def start(self) -> None:
        Thread(target=self._server.serve_forever).start()

    def stop(self) -> None:
        self._server._stoppable_signal.set()


def create_server(params: ServerParameters) -> IServer:
    app = App(middleware=CORSMiddleware(allow_origins='*', allow_credentials='*'))
    app.add_route('/game', GameResource(params))
    app.add_route('/players', PlayersResource())

    logging.getLogger('werkzeug').setLevel(logging.ERROR)

    return SimpleServer(StoppableServer('localhost', params.port, app))
