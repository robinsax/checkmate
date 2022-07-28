import { Color, PieceType, Piece, Game, Move, State } from './types';

interface MoveFormat {
    move: string;
    piece: string;
    taken?: string;
    castle?: string;
    promo?: string;
}

interface StateFormat {
    board: string[];
    active: string;
    moves: MoveFormat[];
    history: MoveFormat[];
    end: { winner: string, condition: string } | null;
}

const PIECE_TYPE_LOOKUP: { [ext: string]: PieceType } = {
    P: 'pawn',
    K: 'king',
    Q: 'queen',
    B: 'bishop',
    R: 'rook',
    N: 'knight'
};
const PIECE_TYPE_REVERSE_LOOKUP: { [key: string]: string } = (
    Object.keys(PIECE_TYPE_LOOKUP).reduce((result, key) => ({
        ...result, [PIECE_TYPE_LOOKUP[key]]: key
    }), {})
);

const COLOR_LOOKUP: { [ext: string]: Color } = {
    w: 'white',
    b: 'black'
};

export class RemoteGameDriver implements Game {
    private url: string;
    private currentState: State | null;
    private gameId: string | null;
    private token: string;

    static async connect(host: string, port: number): Promise<Game> {
        const instance = new RemoteGameDriver([host, port].join(':'));
        await instance.initialize();
        return instance;
    }
    
    private constructor(url: string) {
        this.url = url;

        this.currentState = null;
        this.gameId = null;
        this.token = '';
    }

    private async joinGame(): Promise<string> {
        const login = await fetch(this.url + '/v1/auth', {
            method: 'POST',
            body: JSON.stringify({
                name: 'foo'
            }),
            headers: {
                'Content-Type': 'application/json'
            }
        });

        this.token = (await login.json()).token;

        const creation = await fetch(this.url + '/v1/lobby', {
            method: 'POST',
            body: JSON.stringify({
                game: 'new'
            }),
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + this.token
            }
        });

        return (await creation.json()).id;
    }

    private async fetchState() {
        const fetched = await fetch(this.url + '/v1/games/' + this.gameId, {
            headers: {
                'Authorization': 'Bearer ' + this.token
            }
        });

        this.hydrateStateFrom(await fetched.json() as StateFormat);
    }

    private async initialize(): Promise<void> {
        this.gameId = await this.joinGame();

        await this.fetchState();
    }

    state(): State {
        return this.currentState as State;
    }

    async restart(): Promise<boolean> {
        this.gameId = await this.joinGame();

        await this.fetchState();
        return true;
    }

    async takeTurn(move: Move): Promise<boolean> {
        let moveStr = [move.from, move.to].join('');
        if (move.promotion) {
            console.log(moveStr, PIECE_TYPE_REVERSE_LOOKUP, move);
            moveStr += PIECE_TYPE_REVERSE_LOOKUP[move.promotion];
        };
        const resp = await fetch(this.url + '/v1/games/' + this.gameId, {
            method: 'PUT',
            body: JSON.stringify({ move: moveStr }),
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + this.token
            }
        });

        this.hydrateStateFrom(await resp.json() as StateFormat);
        return true;
    }

    private hydrateStateFrom(input: StateFormat) {
        const parsePiece = (input: string): Piece => ({
            type: PIECE_TYPE_LOOKUP[input[1]],
            color: COLOR_LOOKUP[input[0]]
        });

        const parseMoveStr = (input: string) => [input.substring(0, 2), input.substring(2)];

        const board = input.board.map<[string, Piece]>(input => [
            input.substring(2), parsePiece(input.substring(0, 2))
        ]);

        const parseMove = (input: MoveFormat): Move => {
            const [from, to] = parseMoveStr(input.move);
            let castle: Move['castle'] = null;
            if (input.castle) {
                const [cfrom, cto] = parseMoveStr(input.castle);

                castle = { from: cfrom, to: cto };
            }

            return {
                from, to, castle,
                piece: parsePiece(input.piece),
                taken: input.taken ? parsePiece(input.taken) : null,
                promotion: input.promo ? PIECE_TYPE_LOOKUP[input.promo] : null
            };
        };
       
        const { active, moves, history } = input;

        this.currentState = {
            board,
            active: COLOR_LOOKUP[active],
            moves: moves.map(parseMove),
            history: history.map(parseMove),
            result: (
                input.end ?
                    {
                        winner: COLOR_LOOKUP[input.end.winner] || null,
                        condition: input.end.condition
                    }
                    :
                    null
            )
        };
    }
}