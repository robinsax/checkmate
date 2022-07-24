from ..bases import IPlayer, IGame

class Human(IPlayer):

    def name(self) -> str:
        return 'human'

    def take_turn(self, game: IGame) -> None:
        return
