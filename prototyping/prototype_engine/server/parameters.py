from typing import Callable

from ..game import IGame

class ServerParameters:
    port: int
    get_game: Callable[[], IGame]
    new_game: Callable[[str, str], None]

    def __init__(
        self, port: int = None, get_game: Callable[[], IGame] = None,
        new_game: Callable[[str, str], None] = None
    ) -> None:
        self.port = port
        self.get_game = get_game
        self.new_game = new_game
