from ...serialization import ISerializable, SerializedInput, DeserializeError

from ..bases import IPlayer

from .human import Human
from .heuristic_random import HeuristicRandom

_player_types = (
    Human(), HeuristicRandom()
)

def player_for_name(name: str) -> IPlayer:
    for check in _player_types:
        if check.name() == name:
            return check
    return None

class AbstractPlayer(ISerializable):
    player: IPlayer

    def __init__(self, player: IPlayer = None) -> None:
        super().__init__()

        self.player = player

    def serialize(self) -> str:
        return self.player.name()

    def deserialize(self, data: 'SerializedInput') -> None:
        self.player = player_for_name(data.as_str())
        if not self.player:
            raise DeserializeError('invalid player: %s'%data.as_str())
