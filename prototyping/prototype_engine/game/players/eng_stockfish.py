from stockfish import Stockfish

from ...model import Move

from ..bases import IPlayer, IGame

class StockfishIntegrationError(BaseException):
    pass

class StockfishPlayer(IPlayer):
    _stockfish: Stockfish
    _game: IGame

    def __init__(self) -> None:
        super().__init__()

        self._stockfish = None
        self._game = None

    def name(self) -> str:
        return 'stockfish'

    def is_human(self) -> bool:
        return True

    def _get_stockfish(self) -> Stockfish:
        if not self._stockfish:
            self._stockfish = Stockfish(path='../stockfish_15_win_x64_avx2/stockfish_15_x64_avx2.exe')
        return self._stockfish

    def take_turn(self, game: IGame) -> None:
        alg_moves = list()
        def add_alg_move(move: Move):
            alg_moves.append(''.join((str(move.from_position), str(move.to_position))))

        move_history = game.board.move_history
        if len(move_history) >= 2:
            add_alg_move(move_history[-2])
        if len(move_history) >= 1:
            add_alg_move(move_history[-1])

        stockfish = self._get_stockfish()
        if game is not self._game:
            stockfish.set_position(alg_moves)
            self._game = game
        else:
            stockfish.make_moves_from_current_position(alg_moves)
        alg_best_move = stockfish.get_best_move_time(500)

        internal_moves = game.board.legal_moves(game.turn)
        for move in internal_moves:
            if move.from_position == alg_best_move[:2] and move.to_position == alg_best_move[2:]:
                game.make_move(move)
                return

        raise StockfishIntegrationError()
