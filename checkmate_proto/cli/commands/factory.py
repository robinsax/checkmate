from typing import List

from ..bases import ICLI, ICLICommand

from .new_game import NewGameCommand
from .load_game import LoadGameCommand
from .save_game import SaveGameCommand
from .exit import ExitCommand
from .moves import MovesCommand
from .show_board import ShowBoardCommand
from .move import MoveCommand
from .debug_piece import DebugPieceCommand

def create_commands() -> List[ICLICommand]:
    return (
        NewGameCommand(),
        LoadGameCommand(),
        SaveGameCommand(),
        ExitCommand(),
        MovesCommand(),
        MoveCommand(),
        ShowBoardCommand(),
        DebugPieceCommand()
    )
