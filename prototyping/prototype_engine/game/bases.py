from typing import Tuple

from ..serialization import ISerializable
from ..model import Color, Board, Move, Piece, Position

class IPlayer(ISerializable):

    def take_turn(self, game: 'IGame') -> None:
        raise NotImplementedError('IPlayer.take_turn(game)')

class IGame(ISerializable):
    
    @property
    def active_player(self) -> IPlayer:
        raise NotImplementedError('IGame.active_player')

    @property
    def board(self) -> Board:
        raise NotImplementedError('IGame.board')

    @property
    def turn(self) -> Color:
        raise NotImplementedError('IGame.turn')

    def set_players(self, players: Tuple[IPlayer, IPlayer]) -> None:
        raise NotImplementedError('IGame.set_players(players)')

    def as_move(self, piece: Piece, position: Position) -> Move:
        raise NotImplementedError('IGame.as_move(piece, position)')

    def make_move(self, move: Move) -> None:
        raise NotImplementedError('IGame.make_move(move)')

    def start(self) -> None:
        raise NotImplementedError('IGame.start()')
