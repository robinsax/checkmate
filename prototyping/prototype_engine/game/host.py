import time

from typing import Tuple

from ..serialization import SerializedInput
from ..model import Color, Board, Move, Position, Piece

from .bases import IGame, IPlayer
from .exceptions import GameStateError

class HostGame(IGame):
    _board: Board
    _turn: Color
    _players: IPlayer

    @classmethod
    def new(cls, players: Tuple[IPlayer, IPlayer]):
        ranks = ('1', '2', '3', '4', '5', '6', '7', '8')
        files = ('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h')

        def pawn_rank(rank: str, color: Color):
            return (
                (Position(rank, file), Piece.of_type('pawn', color)) for file in files
            )

        def back_rank(rank: str, color: Color):
            return (
                (Position(rank, 'a'), Piece.of_type('rook', color)),
                (Position(rank, 'b'), Piece.of_type('knight', color)),
                (Position(rank, 'c'), Piece.of_type('bishop', color)),
                (Position(rank, 'd'), Piece.of_type('queen', color)),
                (Position(rank, 'e'), Piece.of_type('king', color)),
                (Position(rank, 'f'), Piece.of_type('bishop', color)),
                (Position(rank, 'g'), Piece.of_type('knight', color)),
                (Position(rank, 'h'), Piece.of_type('rook', color))
            )

        return cls(
            Board.create_with(
                ranks, files, (
                    *back_rank('8', Color.black),
                    *pawn_rank('7', Color.black),
                    *pawn_rank('2', Color.white),
                    *back_rank('1', Color.white)
                )
            ),
            Color.white, players
        )

    def __init__(
        self, board: Board = None, turn: Color = None,
        players: Tuple[IPlayer, IPlayer] = None
    ) -> None:
        super().__init__()

        self._board = board
        self._turn = turn
        self._players = players

    @property
    def active_player(self) -> IPlayer:
        return self._players[int(self._turn is Color.black)]

    @property
    def turn(self) -> Color:
        return self._turn

    @property
    def board(self) -> Board:
        return self._board

    def set_players(self, players: Tuple[IPlayer, IPlayer]) -> None:
        self._players = players

    def as_move(self, piece: Piece, position: Position):
        for move in self.board.legal_moves(piece.color):
            if move.piece == piece and move.to_position == position:
                return move

        raise GameStateError('invalid move')

    def start(self) -> None:
        self.active_player.take_turn(self)

    def make_move(self, move: Move) -> None:
        if move.piece.color is not self.turn:
            raise GameStateError('not your turn')

        self._board.apply_move(move)
        self._turn = Color.black if self.turn is Color.white else Color.white

        self.active_player.take_turn(self)

    def serialize(self) -> dict:
        return {
            'board': self.board,
            'turn': self.turn
        }

    def deserialize(self, data: SerializedInput) -> None:
        self._board = data.dict_lookup('board').as_instance(Board)
        self._turn = Color.white if data.dict_lookup('turn').as_str() == 'white' else Color.black

    def __str__(self):
        def taken_pieces_str(color: Color):
            taken_names = list()

            for move in self.board.move_history:
                if move.taken and move.taken.color is color:
                    taken_names.append(str(move.taken))

            if not len(taken_names):
                return str()

            return ''.join((
                '\n○' if color is Color.white else '\n●', ''.join(taken_names)
            ))

        return 'turn %s, %s to move%s%s\n\n%s'%(
            len(self.board.move_history) + 1, self.turn.value,
            taken_pieces_str(Color.white), taken_pieces_str(Color.black),
            str(self.board)
        )
