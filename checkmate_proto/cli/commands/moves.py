from typing import List

from ...model import IPiece

from ..bases import ICLIArgument
from ..arguments import PieceCLIArgument

from .common import BaseCLICommand

class MovesCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('moves',)

    def arguments(self) -> List[ICLIArgument]:
        return (PieceCLIArgument(),)

    def _overlay_char(self, rank_dir: int, file_dir: int):
        if rank_dir == -1:
            return '↓' if not file_dir else ('↘' if file_dir > 0 else '↙')
        if rank_dir == 1:
            return '↑' if not file_dir else ('↗' if file_dir > 0 else '↖')
        return '→' if file_dir > 0 else '←'

    def command(self, piece: IPiece) -> str:
        board = self.game.board
        moves = piece.moves(board)

        repr_rows = str(board).split('\n')
        cur_rank_idx = board.ranks.index(piece.position.rank)
        cur_file_idx = board.files.index(piece.position.file)
        for move in moves:
            rank_idx = board.ranks.index(move.rank)
            file_idx = board.files.index(move.file)

            overlay_char = self._overlay_char(
                min(max(rank_idx - cur_rank_idx, -1), 1),
                min(max(file_idx - cur_file_idx, -1), 1)
            )
            col_char_idx = len(board.ranks) - 1 - rank_idx
            row_char_idx = ((file_idx + 1) * 2)
            repr_rows[col_char_idx] = ''.join((
                repr_rows[col_char_idx][:row_char_idx],
                overlay_char,
                repr_rows[col_char_idx][row_char_idx + 1:]
            ))

        return '\n'.join(repr_rows)
