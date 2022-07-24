from typing import List, Tuple

from ...model import Position, Piece, Move
from ...game import GameStateError, IGame

from ..bases import ICLIArgument
from ..arguments import PieceCLIArgument, OptionalCLIArgument, PositionCLIArgument
from ..exception import CLIInputError

from .common import BaseCLICommand

class MovesCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('moves',)

    def arguments(self) -> List[ICLIArgument]:
        return (OptionalCLIArgument(inner=PieceCLIArgument()),)

    def _overlay_char(self, rank_dir: int, file_dir: int):
        if rank_dir == -1:
            return '↓' if not file_dir else ('↘' if file_dir > 0 else '↙')
        if rank_dir == 1:
            return '↑' if not file_dir else ('↗' if file_dir > 0 else '↖')
        return '→' if file_dir > 0 else '←'

    def _show_piece_moves_board(
        self, game: IGame, moves: List[Move], piece_info: Tuple[Position, Piece]
    ) -> Tuple[str, List[Move]]:
        board = game.board

        view_matrix = board.view_matrix(with_legend=True)

        cur_rank_idx = board.ranks.index(piece_info[0].rank)
        cur_file_idx = board.files.index(piece_info[0].file)
        
        piece_moves = list()
        for move in moves:
            if move.piece != piece_info[1]:
                continue

            piece_moves.append(move)

            rank_idx = board.ranks.index(move.to_position.rank)
            file_idx = board.files.index(move.to_position.file)

            view_matrix[len(board.ranks) - 1 - rank_idx][file_idx + 1] = (
                self._overlay_char(
                    min(max(rank_idx - cur_rank_idx, -1), 1),
                    min(max(file_idx - cur_file_idx, -1), 1)
                )
            )

        return (
            '\n'.join(' '.join(rank) for rank in view_matrix), piece_moves
        )

    def command(self, piece_info: Tuple[Position, Piece] = None) -> str:
        game = self.cli.expect_state('game', IGame)
        moves = game.board.legal_moves(game.turn)
        board = str()
        if piece_info:
            board, moves = self._show_piece_moves_board(game, moves, piece_info)

        return '\n'.join((board, *(str(move) for move in moves)))

class MoveCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('move',)

    def arguments(self) -> List[ICLIArgument]:
        return (PieceCLIArgument(), PositionCLIArgument())

    def command(self, piece_info: Tuple[Position, Piece], position: Position) -> str:
        game = self.cli.expect_state('game', IGame)
        if not game.active_player.is_human():
            raise CLIInputError('not your turn')

        try:
            game.make_move(game.as_move(piece_info[1], position))
        except GameStateError as err:
            raise CLIInputError('nope, %s'%str(err))

        return str(game)
