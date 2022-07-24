from ..bases import IPlayer, IGame

class Human(IPlayer):

    def name(self) -> str:
        return 'human'

    def is_human(self) -> bool:
        return True

    def take_turn(self, game: IGame) -> None:
        return
