from typing import Tuple, List

from ..serialization import SerializedInput
from ..model import Color, Board, Move, Position, Piece

from .bases import GameResult, IGame, IPlayer, ResultCondition
from .exceptions import GameStateError

class HostGame(IGame):
    _board: Board
    _turn: Color
    _players: List[IPlayer]
    _result: GameResult

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
        players: Tuple[IPlayer, IPlayer] = None,
        result: GameResult = None
    ) -> None:
        super().__init__()

        self._board = board
        self._turn = turn
        self._players = players
        self._result = result

    @property
    def active_player(self) -> IPlayer:
        return self._players[int(self._turn is Color.black)]

    @property
    def result(self) -> GameResult:
        return self._result

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
        if not self.result:
            self.active_player.take_turn(self)

    def _check_result(self) -> GameResult:
        for piece in self._board.state.values():
            if piece.name not in ('♔', '♚'):
                break
        else:
            return GameResult(
                winner=None,
                condition=ResultCondition.insufficient_materiel
            )

        moves = self._board.legal_moves(self._turn)
        if not len(moves):
            return GameResult(
                winner=Color.inverse(self._turn),
                condition=ResultCondition.checkmate
            )

        return None

    def make_move(self, move: Move) -> None:
        if self.result:
            raise GameStateError('game over')
        if move.piece.color is not self.turn:
            raise GameStateError('not your turn')

        self._board.apply_move(move)
        self._turn = Color.inverse(self._turn)

        self._result = self._check_result()
        if not self._result:
            self.active_player.take_turn(self)

    def serialize(self) -> dict:
        return {
            'state': {
                'board': self.board,
                'turn': self.turn,
                'result': self.result,
                'legal_moves': self.board.legal_moves(self.turn)
            },
            'players': {
                'white': self._players[0].name(),
                'black': self._players[1].name()
            }
        }

    def deserialize(self, data: SerializedInput) -> None:
        state_dict = data.dict_lookup('state')

        self._board = state_dict.dict_lookup('board').as_instance(Board)
        self._turn = state_dict.dict_lookup('turn').as_instance(Color)

        result_repr = state_dict.dict_lookup('result', allow_empty=True)
        if result_repr:
            self._result = result_repr.as_instance(GameResult)

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

        result_str = ''
        if self._result:
            result_str = '\n\n%s wins by %s'%(
                self._result.winner.value, self._result.condition.value
            )

        return 'turn %s, %s to move%s%s\n\n%s%s'%(
            len(self.board.move_history) + 1, self.turn.value,
            taken_pieces_str(Color.white), taken_pieces_str(Color.black),
            str(self.board), result_str
        )
