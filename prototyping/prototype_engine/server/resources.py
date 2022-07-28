from falcon import Request, Response

from ..model import Move
from ..game import GameStateError, all_available_player_names
from ..serialization import ISerializable, ISerializer, SerializedInput, JSONSerializer

from .parameters import ServerParameters

class GameTemplate(ISerializable):
    white: str
    black: str

    def __init__(self) -> None:
        self.white = None
        self.black = None
    
    def serialize(self) -> dict:
        raise NotImplementedError()

    def deserialize(self, data: SerializedInput) -> None:
        self.white = data.dict_lookup('white').as_str()
        self.black = data.dict_lookup('black').as_str()

class PlayersResource:
    _serializer: ISerializer

    def __init__(self) -> None:
        self._serializer = JSONSerializer()

    def on_get(self, req: Request, resp: Response) -> None:
        resp.data = self._serializer.serialize(all_available_player_names())
        resp.content_type = 'application/json'

class GameResource:
    _params: ServerParameters
    _serializer: ISerializer

    def __init__(self, params: ServerParameters) -> None:
        self._params = params
        self._serializer = JSONSerializer()

    def on_get(self, req: Request, resp: Response) -> None:
        game = self._params.get_game()
        resp.data = self._serializer.serialize({
            'legal_moves': game.board.legal_moves(game.turn),
            **game.serialize()
        })
        resp.content_type = 'application/json'

    def on_put(self, req: Request, resp: Response) -> None:
        move = self._serializer.deserialize(req.bounded_stream.read(), Move)

        try:
            self._params.get_game().make_move(move)
        except GameStateError as err:
            resp.content_type = 'application/json'
            resp.data = {'error': str(err)}
            return

        self.on_get(req, resp)

    def on_post(self, req: Request, resp: Response) -> None:
        template = self._serializer.deserialize(req.bounded_stream.read(), GameTemplate)
        
        self._params.new_game(template.white, template.black)

        self.on_get(req, resp)
