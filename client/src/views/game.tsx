import './game.scss';

import { Component, createSignal, For, onMount } from 'solid-js';

import { Board, Game, Piece, Move, Color, GameState } from '../game';
import { RemotePrototypeHostDriver } from '../game/drivers';
import { navigate, QueryMap } from '../router';

export interface GameViewProps {
    query: QueryMap;
}

interface BoardViewProps {
    board: Board;
    allMoves: Move[];
    makeMove: (piece: Piece, position: string) => void;
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

interface ActiveDrag {
    piece: Piece;
    dragX: number;
    dragY: number;
}

const BOARD_SIZE_PX = 4 * 16;

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
            { props.piece.name }
        </div>
    )
};

const BoardView: Component<BoardViewProps> = props => {
    let viewEl: HTMLElement | null = null;

    const [hovered, setHovered] = createSignal<string | null>(null);
    const [drag, setDrag] = createSignal<ActiveDrag | null>(null);

    const pieceAtPosition = (position: string): Piece | null => {
        const pieceInfo = props.board.pieces.filter(check => (
            check[0] == position
        ))[0];
        if (!pieceInfo) return null;

        return pieceInfo[1];
    };

    const updateDragFromEvent = (piece: Piece, event: MouseEvent) => {
        if (!viewEl) return;
        const viewBounds = viewEl.getBoundingClientRect();

        setDrag({
            piece,
            dragX: event.pageX - viewBounds.x,
            dragY: event.pageY - viewBounds.y
        });
    };

    const handleMouseDown = (event: MouseEvent) => {
        const currentHover = hovered();
        if (!currentHover) return;

        const piece = pieceAtPosition(currentHover);
        if (!piece) return;

        updateDragFromEvent(piece, event);
    };

    const handleMouseUp = () => {
        const currentHovered = hovered();
        const currentDrag = drag();
        if (!currentDrag || !currentHovered) return;

        props.makeMove(currentDrag.piece, currentHovered);
        setDrag(null);
    };

    const handleMouseMove = (event: MouseEvent) => {
        const currentDrag = drag();
        if (!currentDrag) return;

        updateDragFromEvent(currentDrag.piece, event);
    };

    const positionIndexes = (position: string): [number, number] => ([
        props.board.ranks.length - (props.board.ranks.indexOf(position[1]) + 1),
        props.board.files.indexOf(position[0])
    ]);

    const movesForPiece = (piece: Piece | null) => (
        !piece ? [] : props.allMoves.filter(check => check.piece.id == piece.id)
    );

    const isMoveTarget = (position: string) => {
        const currentHovered = hovered();
        const currentDrag = drag();
        if (!currentHovered && !currentDrag) return;

        const piece = (
            currentDrag ?
                currentDrag.piece : pieceAtPosition(currentHovered as string)
        );
        if (!piece) return;

        return (
            movesForPiece(piece).filter(check => (
                check.to == position
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
            <For each={ [...props.board.ranks].reverse() }>{ (rank, i) => (
                <div class="rank">
                    <div
                        class="rank-label"
                        style={ { top: (BOARD_SIZE_PX * i()) + 'px' } }
                    >
                        { rank }
                    </div>
                    <For each={ props.board.files }>{ (file, j) => (
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
                <For each={ props.board.files }>{ file => (
                    <div class="file-label">{ file }</div>
                ) }</For>
            </div>
            <For each={ props.board.pieces }>{ ([position, piece]) => (([rankIndex, fileIndex]) => (
                <PieceView
                    { ...{rankIndex, fileIndex, piece, position} }
                    hovered={ hovered() == position }
                    drag={ drag()?.piece.id == piece.id ? drag() : null }
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
                    { move.piece.name } { move.from } → { move.to }
                    { move.taken ? (' (takes ' + move.taken.name + ')') : '' }
                    { move.castle_other ? ( ' (castles)') : '' }
                </div>
            ) }</For>
        </div>
    );
};

export const GameView: Component<GameViewProps> = props => {
    const [gameState, setGameState] = createSignal<GameState | null>(null);

    let takeTurn: (move: Move) => Promise<void> = async () => {};

    const takeTurnFromDrop = (piece: Piece, position: string) => {
        const moves = (gameState() as GameState).legal_moves;

        let foundMove: Move | null = null;
        for (const move of moves) {
            if (move.to == position && move.piece.id == piece.id) {
                foundMove = move;
                break;
            }
        }
        if (!foundMove) return;

        takeTurn(foundMove);
    };

    onMount(async () => {
        const { query: { host, port } } = props;
        if (!host || !port) {
            navigate('/connect');
            return;
        }

        let game: Game | null = null;
        try {
            game = await RemotePrototypeHostDriver.connect(host, +port);
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
    });

    return (
        <div class="game-view">
            { ((state: GameState | null) => (!state ?
                <div class="loading">loading...</div>
                :
                <div class="game-area">
                    <div class="game-main-area">
                        <BoardView
                            board={ state.board }
                            allMoves={ state.legal_moves }
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
                                    { JSON.stringify(state.active_player) } to move
                                </span>
                            }
                        </div>
                        <MovesView history={ state.board.history }/>
                    </div>
                </div>
            ))(gameState()) }
        </div>
    );
};
