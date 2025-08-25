import { JSX, KeyboardEventHandler, useCallback, useEffect, useState, useSyncExternalStore } from 'react'

import { Board as WasmBoard, Hint as WasmHint, Cell as WasmCell } from './pkg'
import Cell, { HighlightContext } from './Cell.tsx'

import './Board.css'
import Hint from './Hint.tsx';

class ExternalBoard {
    board: WasmBoard;
    callback: null | (() => void);
    cache: null | WasmCell[];

    constructor(slug: string | null) {
        if (slug !== null && slug != "") {
            this.board = WasmBoard.from_slug(slug);
        } else {
            this.board = new WasmBoard();
        }

        this.callback = null;
    }

    subscribe(cb) {
        this.callback = cb;
        return () => {
            this.callback = null;
        }
    }

    fetch() {
        if (!this.cache) {
            this.cache = this.board.to_js();
        }
        return this.cache;
    }

    signal() {
        this.cache = null;
        history.pushState(this.board.to_slug(), "", "#" + this.board.to_slug());
        if (this.callback !== null) {
            this.callback();
        }
    }
}

function Board() {
    const [board, setBoard] = useState(() => {
        if (window.location.hash.startsWith("#")) {
            return new ExternalBoard(window.location.hash.substring(1));
        } else {
            return new ExternalBoard("");
        }
    });
    const subscribe = useCallback((cb) => board.subscribe(cb), [board]);
    const getSnapshot = useCallback(() => board.fetch(), [board]);
    const cells = useSyncExternalStore(subscribe, getSnapshot);

    const [highlight, setHighlight] = useState(null as string | null);
    const [selectedHint, setSelectedHint] = useState(null as WasmHint | null);
    const [kbFocusIndex, setKbFocusIndex] = useState(-1);

    useEffect(
        function () {
            const listener = function (e) {
                setBoard(new ExternalBoard(e.state));
            };
            window.addEventListener('popstate', listener);
            return () => window.removeEventListener('popstate', listener);
        },
        [setBoard]
    )

    const children: JSX.Element[] = [];
    for (let x = 0; x < 9; ++x) {
        for (let y = 0; y < 9; ++y) {
            const index = x * 9 + y;
            const onUpdate = function (value) {
                board.board.mark_cell_solved(x, y, value);
                board.signal();
            }

            const makeGo: (number) => (() => void) = function (step) {
                return function () {
                    const newIndex = index + step;
                    if (newIndex < 0 || newIndex >= 81) {
                        // trying to navigate outside the board, so just ignore it.
                        return;
                    }
                    setKbFocusIndex(newIndex);
                }
            }

            let cause: number[] | null = null;
            let effect: number[] | null = null;
            if (selectedHint !== null) {
                // TODO: do something better than linear search
                for (const causeLocation of selectedHint.cause) {
                    if (causeLocation.location.x === x && causeLocation.location.y === y) {
                        cause = Array.from(causeLocation.digit ?? []);
                    }
                }

                for (const effectLocation of selectedHint.effect) {
                    if (effectLocation.location.x === x && effectLocation.location.y === y) {
                        effect = Array.from(effectLocation.digit ?? []);
                    }
                }
            }

            children.push(
                // TODO: is the key *really* needed, since this list never actually changes?
                // the warning sent me to https://react.dev/learn/rendering-lists#why-does-react-need-keys
                <Cell
                    key={index}
                    kbFocusRequested={index === kbFocusIndex}
                    state={cells[index]}
                    cause={cause}
                    effect={effect}
                    onUpdate={onUpdate}
                    goLeft={makeGo(-1)}
                    goRight={makeGo(+1)}
                    goUp={makeGo(-9)}
                    goDown={makeGo(+9)}
                />
            )
        }
    }

    const onLeave = function () {
        setSelectedHint(null);
    }

    const hints: JSX.Element[] = [];
    for (const hint of board.board.hints()) {
        const onEnter = function () {
            setSelectedHint(hint);
        }
        const commit = function () {
            board.board.apply(hint);
            board.signal();
            setSelectedHint(null);
        }
        hints.push(
            <Hint
                key={hint.identity}
                hint={hint}
                onEnter={onEnter}
                onLeave={onLeave}
                commit={commit}
            />
        );
    }

    const onKeyDown: KeyboardEventHandler = function (e) {
        if (e.key == "z" && (e.ctrlKey || e.metaKey)) {
            history.back();
            e.preventDefault();
            return;
        }
    };

    const onKeyUp: KeyboardEventHandler = function (e) {
        if (e.key == "Escape") {
            setHighlight(null);
            e.stopPropagation();
            return;
        }

        if (e.shiftKey && e.code.startsWith("Digit") && e.code.length == 6) {
            const value = parseInt(e.code.substring(5));
            if (!isNaN(value)) {
                setHighlight("" + value);
                e.stopPropagation();
                return;
            }
        }
    }

    return <div className="game">
        <div className='board' onKeyDown={onKeyDown} onKeyUp={onKeyUp}>
            <HighlightContext value={highlight}>
                {children}
            </HighlightContext>
        </div>
        <div className="hints">
            {hints}
        </div>
    </div>;
}

export default Board
