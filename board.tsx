import { JSX, KeyboardEventHandler, RefObject, useRef, useState } from 'react'

import { Board as WasmBoard, Hint as WasmHint } from './pkg'
import Cell, { HighlightContext } from './Cell.tsx'

import './Board.css'
import Hint from './Hint.tsx';

function Board() {
    // this is the "Avoiding recreating the ref contents" example, but with an additional type hint
    // so that TypeScript can know what I'm *going* to put in it.
    const boardRef: RefObject<WasmBoard | null> = useRef(null);
    if (boardRef.current === null) {
        boardRef.current = new WasmBoard();
    }

    const [cells, setCells] = useState(() => boardRef.current!.to_js());
    const [highlight, setHighlight] = useState(null as string | null);
    const [selectedHint, setSelectedHint] = useState(null as WasmHint | null);

    const children: JSX.Element[] = [];
    const childRefs: RefObject<HTMLDivElement | null>[] = [];
    for (let x = 0; x < 9; ++x) {
        for (let y = 0; y < 9; ++y) {
            const index = x * 9 + y;
            const onUpdate = function (value) {
                boardRef.current!.mark_cell_solved(x, y, value);
                setCells(boardRef.current!.to_js());
            }

            const makeGo: (number) => (() => void) = function (step) {
                return function () {
                    const newIndex = index + step;
                    if (newIndex < 0 || newIndex >= 81) {
                        // trying to navigate outside the board, so just ignore it.
                        return;
                    }

                    childRefs[newIndex]?.current?.focus();
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

            // TODO: I *think* useRef depends on the order it's called, but since I always call it
            // exactly 81 times then this should be safe.  Is there a better way?
            const ref = useRef(null);
            childRefs.push(ref);
            children.push(
                // TODO: is the key *really* needed, since this list never actually changes?
                // the warning sent me to https://react.dev/learn/rendering-lists#why-does-react-need-keys
                <Cell
                    key={index}
                    ref={ref}
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
    for (const hint of boardRef.current.hints()) {
        const onEnter = function () {
            setSelectedHint(hint);
        }
        hints.push(
            <Hint
                // TODO: come up with a reasonable key here to quiet the React warning
                hint={hint}
                onEnter={onEnter}
                onLeave={onLeave}
            />
        );
    }

    const onKeyDown: KeyboardEventHandler = function (e) {
        if (e.key == "z" && (e.ctrlKey || e.metaKey)) {
            boardRef.current!.undo();
            setCells(boardRef.current!.to_js());
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
