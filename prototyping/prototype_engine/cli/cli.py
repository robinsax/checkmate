from typing import List, Mapping, Any, TypeVar, Type

from .exception import CLIInputError
from .bases import ICLICommand

TStateType = TypeVar('TStateType')

class CLI:
    _state: dict
    _command_lookup: Mapping[str, ICLICommand]

    def __init__(self, commands: List[ICLICommand]) -> None:
        super().__init__()
        
        self._state = dict()

        self._init_commands(commands)

    def _init_commands(self, commands: List[ICLICommand]) -> None:
        self._command_lookup = dict()
        
        for command in commands:
            command.set_cli(self)

            verbs = command.verbs()
            for verb in verbs:
                self._command_lookup[verb] = command

    def get_state(self, key: str, type: Type[TStateType]) -> TStateType:
        value = self._state.get(key)
        if value and not isinstance(value, type):
            raise CLIInputError('invalid %s'%key)

        return value

    def expect_state(self, key: str, type: Type[TStateType]) -> TStateType:
        if key not in self._state:
            raise CLIInputError('no %s'%key)
        
        return self.get_state(key, type)

    def set_state(self, key: str, value: Any) -> None:
        if value is None:
            if key in self._state:
                del self._state[key]
            return

        self._state[key] = value

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
                print('"exit" to exit')
