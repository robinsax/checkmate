from typing import Tuple

from ..serialization import ISerializable
from ..model import Color, Board, Move, Piece, Position

class IPlayer(ISerializable):

    def name(self) -> str:
        raise NotImplementedError()

    def take_turn(self, game: 'IGame') -> None:
        raise NotImplementedError()

class IGame(ISerializable):
    
    @property
    def active_player(self) -> IPlayer:
        raise NotImplementedError()

    @property
    def board(self) -> Board:
        raise NotImplementedError()

    @property
    def turn(self) -> Color:
        raise NotImplementedError()

    def set_players(self, players: Tuple[IPlayer, IPlayer]) -> None:
        raise NotImplementedError()

    def as_move(self, piece: Piece, position: Position) -> Move:
        raise NotImplementedError()

    def make_move(self, move: Move) -> None:
        raise NotImplementedError()

    def start(self) -> None:
        raise NotImplementedError()
