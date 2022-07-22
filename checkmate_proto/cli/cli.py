import sys

from typing import List, Mapping

from ..serialization import JSONSerializer
from ..game import IGame, create_default_game

from .exception import CLIInputError
from .bases import ICLI, ICLICommand

class CLI(ICLI):
    _game: IGame
    _command_lookup: Mapping[str, ICLICommand]

    def __init__(self, commands: List[ICLICommand]) -> None:
        super().__init__()
        
        self._game = None

        self._init_commands(commands)

    def _init_commands(self, commands: List[ICLICommand]) -> None:
        self._command_lookup = dict()
        
        for command in commands:
            command.set_cli(self)

            verbs = command.verbs()
            for verb in verbs:
                self._command_lookup[verb] = command

    @property
    def game(self) -> IGame:
        return self._game

    def set_game(self, game: IGame) -> None:
        self._game = game

    def _read_input(self):
        return input('> ').split()

    def _tick(self):
        verb, *raw_args = self._read_input()

        if verb not in self._command_lookup:
            raise CLIInputError('what?')

        command = self._command_lookup[verb]

        arg_parsers = command.arguments()
        command_args = list()
        for i, parser in enumerate(arg_parsers):
            value = '<none>' if i >= len(raw_args) else raw_args[i]

            parser.set_cli(self)

            command_args.append(parser.parse(value))

        output = command.command(*command_args)

        print(output)

    def run(self):
        while True:
            try:
                self._tick()
            except CLIInputError as err:
                print(str(err))
            except KeyboardInterrupt:
                print('bye')
                sys.exit(0)
