from typing import List

from ..bases import ICLICommand

from .game import NewGameCommand, LoadGameCommand, SaveGameCommand
from .exit import ExitCommand
from .moves import MovesCommand, MoveCommand
from .show_board import ShowBoardCommand
from .debug_piece import DebugPieceCommand
from .hosting import HostGameCommand, DehostGameCommand

def create_commands() -> List[ICLICommand]:
    return (
        NewGameCommand(),
        LoadGameCommand(),
        SaveGameCommand(),
        ExitCommand(),
        MovesCommand(),
        MoveCommand(),
        ShowBoardCommand(),
        DebugPieceCommand(),
        HostGameCommand(),
        DehostGameCommand()
    )
