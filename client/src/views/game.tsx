import './game.scss';

import { Component, createSignal, For, onMount } from 'solid-js';

import { Game, Piece, Move, State, RemoteGameDriver, PieceType, RANKS, FILES } from '../game';
import { navigate, QueryMap } from '../router';

export interface GameViewProps {
    query: QueryMap;
}

interface BoardViewProps {
    board: [string, Piece][];
    moves: Move[];
    makeMove: (drag: ActiveDrag, position: string) => void;
}

interface PieceViewProps {
    rankIndex: number;
    fileIndex: number;
    position: string;
    hovered: boolean;
    drag: ActiveDrag | null;
    piece: Piece;
}

interface MovesViewProps {
    history: Move[];
}

interface MoveChoiceProps {
    choices: Move[];
    makeChoice: (choice: Move) => void;
    cancelChoice: () => void;
}

interface ActiveDrag {
    piece: Piece;
    position: string;
    dragX: number;
    dragY: number;
}

const BOARD_SIZE_PX = 4 * 16;

const PIECE_DISPLAY_LOOKUP: { [key in PieceType]: [string, string] } = {
    pawn: ['♙', '♟'],
    bishop: ['♗', '♝'],
    rook: ['♖', '♜'],
    knight: ['♘', '♞'],
    queen: ['♕', '♛'],
    king: ['♔', '♚']
};

const pieceChar = (piece: Piece) => {
    const set = PIECE_DISPLAY_LOOKUP[piece.type];

    const index = (
        (window as any).theme == 'dark' ?
            piece.color == 'white' ? 1 : 0
            :
            piece.color == 'black' ? 1 : 0
    );

    return set[index];
};

const PieceView: Component<PieceViewProps> = props => {
    return (
        <div
            classList={ { piece: true, hovered: props.hovered, dragging: !!props.drag } }
            style={ {
                top: (
                    props.drag ? props.drag.dragY : (BOARD_SIZE_PX * props.rankIndex)
                ) + 'px',
                left: (
                    props.drag ? props.drag.dragX : (BOARD_SIZE_PX * props.fileIndex)
                ) + 'px'
            } }
        >
            { pieceChar(props.piece) }
        </div>
    )
};

const BoardView: Component<BoardViewProps> = props => {
    let viewEl: HTMLElement | null = null;

    const [hovered, setHovered] = createSignal<string | null>(null);
    const [drag, setDrag] = createSignal<ActiveDrag | null>(null);

    const pieceAtPosition = (position: string): Piece | null => {
        const pieceInfo = props.board.filter(check => (
            check[0] == position
        ))[0];
        if (!pieceInfo) return null;

        return pieceInfo[1];
    };

    const updateDragFromEvent = (piece: Piece, position: string, event: MouseEvent) => {
        if (!viewEl) return;
        const viewBounds = viewEl.getBoundingClientRect();

        setDrag({
            piece, position,
            dragX: event.pageX - viewBounds.x,
            dragY: event.pageY - viewBounds.y
        });
    };

    const handleMouseDown = (event: MouseEvent) => {
        const currentHover = hovered();
        if (!currentHover) return;

        const piece = pieceAtPosition(currentHover);
        if (!piece) return;

        updateDragFromEvent(piece, currentHover, event);
    };

    const handleMouseUp = () => {
        const currentHovered = hovered();
        const currentDrag = drag();
        if (!currentDrag || !currentHovered) return;

        props.makeMove(currentDrag, currentHovered);
        setDrag(null);
    };

    const handleMouseMove = (event: MouseEvent) => {
        const currentDrag = drag();
        if (!currentDrag) return;

        updateDragFromEvent(currentDrag.piece, currentDrag.position, event);
    };

    const positionIndexes = (position: string): [number, number] => ([
        RANKS.length - (RANKS.indexOf(position[1]) + 1),
        FILES.indexOf(position[0])
    ]);

    const movesForPosition = (position: string) => (
        props.moves.filter(check => check.to == position)
    );

    const isMoveTarget = (position: string) => {
        const currentHovered = hovered();
        const currentDrag = drag();
        if (!currentHovered && !currentDrag) return;

        return (
            movesForPosition(position).filter(check => (
                check.from == (currentDrag?.position || currentHovered)
            )).length > 0
        );
    };

    return (
        <div
            ref={ el => viewEl = el }
            class="board-view"
            onMouseDown={ handleMouseDown }
            onMouseUp={ handleMouseUp }
            onMouseMove={ handleMouseMove }
        >
            <For each={ [...RANKS].reverse() }>{ (rank, i) => (
                <div class="rank">
                    <div
                        class="rank-label"
                        style={ { top: (BOARD_SIZE_PX * i()) + 'px' } }
                    >
                        { rank }
                    </div>
                    <For each={ FILES }>{ (file, j) => (
                        <div
                            classList={ {
                                cell: true,
                                'move-target': isMoveTarget(file + rank)
                            } }
                        >
                            { (i() + j()) % 2 ? 'x' : '' }
                            <div
                                class="cell-hit-target"
                                onMouseEnter={ () => setHovered(file + rank) }
                            />
                        </div>
                    ) }</For>
                </div>
            ) }</For>
            <div class="file-labels">
                <For each={ FILES }>{ file => (
                    <div class="file-label">{ file }</div>
                ) }</For>
            </div>
            <For each={ props.board }>{ ([position, piece]) => (([rankIndex, fileIndex]) => (
                <PieceView
                    { ...{rankIndex, fileIndex, piece, position} }
                    hovered={ hovered() == position }
                    drag={ drag()?.position == position ? drag() : null }
                />
            ))(positionIndexes(position)) }</For>
        </div>
    )
};

const MovesView: Component<MovesViewProps> = props => {
    return (
        <div class="move-history-view">
            <For each={ [...props.history].reverse() }>{ move => (
                <div class="move">
                    { pieceChar(move.piece) } { move.from } → { move.to }
                    { move.taken ? (' (takes ' + pieceChar(move.taken) + ')') : '' }
                    { move.castle ? ( ' (castles)') : '' }
                </div>
            ) }</For>
        </div>
    );
};

const MoveChoiceView: Component<MoveChoiceProps> = props => {
    return (
        <div class="move-choice-view">
            <div class="move-choice-view-bg"/>
            <div class="move-choice-hint">pick one</div>
            <div
                class="move-choice-cancel"
                onClick={ props.cancelChoice }
            >
                x
            </div>
            <For each={ props.choices }>{ item => (
                <div
                    class="move-choice-item"
                    onClick={ () => props.makeChoice(item) }
                >
                    { pieceChar({ type: item.promotion as PieceType, color: (item.piece).color }) }
                </div>
            ) }</For>
        </div>
    );
};

export const GameView: Component<GameViewProps> = props => {
    const [gameState, setGameState] = createSignal<State | null>(null);
    const [moveChoice, setMoveChoice] = createSignal<Move[] | null>(null);

    let takeTurn: (move: Move) => Promise<void> = async () => {};
    let restart: () => Promise<void> = async () => {};

    const takeTurnFromDrop = (drag: ActiveDrag, position: string) => {
        const { moves } = (gameState() as State);

        const foundMoves: Move[] = [];
        for (const move of moves) {
            if (move.to == position && move.from == drag.position) {
                foundMoves.push(move);
            }
        }
        if (!foundMoves.length) return;

        if (foundMoves.length == 1) {
            takeTurn(foundMoves[0]);
            return;
        }

        setMoveChoice(foundMoves);
    };

    const takeTurnFromChoice = (move: Move) => {
        setMoveChoice(null);
        takeTurn(move);
    };

    onMount(async () => {
        const { query: { host, port } } = props;
        if (!host || !port) {
            navigate('/connect');
            return;
        }

        let game: Game | null = null;
        try {
            game = await RemoteGameDriver.connect(host, +port);
        }
        catch (err) { }

        if (!game) {
            navigate('/connect');
            return;
        }

        setGameState(game.state());

        takeTurn = async (move: Move) => {
            if (!game) return;

            const applied = await game.takeTurn(move);
            if (!applied) return;

            setGameState(game.state());
        };
        restart = async () => {
            if (!game) return;

            const applied = await game.restart();
            if (!applied) return;

            setGameState(game.state());
        }
    });

    return (
        <div class="game-view">
            { ((state: State | null) => (!state ?
                <div class="loading">loading...</div>
                :
                <div class="game-area">
                    <div class="game-main-area">
                        { (choices => choices && (
                            <MoveChoiceView
                                choices={ choices }
                                makeChoice={ takeTurnFromChoice }
                                cancelChoice={ () => setMoveChoice(null) }
                            />
                        ))(moveChoice()) }
                        <BoardView
                            board={ state.board }
                            moves={ state.moves }
                            makeMove={ takeTurnFromDrop }
                        />
                    </div>
                    <div class="game-side-area">
                        <div class="active-state">
                            { state.result ?
                                <span>
                                    game over, { JSON.stringify(state.result) }
                                </span>
                                :
                                <span> 
                                    { JSON.stringify(state.active) } to move
                                </span>
                            }
                        </div>
                        <MovesView history={ state.history }/>
                        <div class="game-actions">
                            <button onClick={ restart }>
                                new
                            </button>
                        </div>
                    </div>
                </div>
            ))(gameState()) }
        </div>
    );
};
