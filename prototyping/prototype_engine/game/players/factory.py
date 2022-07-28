from typing import List

from ...serialization import ISerializable, SerializedInput, DeserializeError

from ..bases import IPlayer

from .human import Human
from .heur_random import HeuristicRandom
from .eng_stockfish import StockfishPlayer

_player_types = (
    Human(), HeuristicRandom(), StockfishPlayer()
)

class InvalidPlayerError(BaseException):
    pass

def player_for_name(name: str) -> IPlayer:
    for check in _player_types:
        if check.name() == name:
            return check
    
    raise InvalidPlayerError(name)

def all_available_player_names() -> List[str]:
    names = list()
    for type in _player_types:
        names.append(type.name())    
    return names

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
