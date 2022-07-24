from falcon import Request, Response

from ..model import Move
from ..game import IGame, GameStateError
from ..serialization import ISerializer, JSONSerializer

class GameResource:
    _game: IGame
    _serializer: ISerializer

    def __init__(self, game: IGame) -> None:
        self._game = game
        self._serializer = JSONSerializer()

    def on_get(self, req: Request, resp: Response) -> None:
        resp.content_type = 'application/json'
        resp.data = self._serializer.serialize({
            'legal_moves': self._game.board.legal_moves(self._game.turn),
            **self._game.serialize()
        })

    def on_post(self, req: Request, resp: Response) -> None:
        move = self._serializer.deserialize(req.bounded_stream.read(), Move)

        try:
            self._game.make_move(move)
        except GameStateError as err:
            resp.content_type = 'application/json'
            resp.data = {'error': str(err)}
            return

        self.on_get(req, resp)
