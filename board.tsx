import { JSX, RefObject, useRef, useState } from 'react'

import { Board as WasmBoard } from './pkg'
import Cell from './Cell.tsx'

import './Board.css'

function Board() {
    // this is the "Avoiding recreating the ref contents" example, but with an additional type hint
    // so that TypeScript can know what I'm *going* to put in it.
    const boardRef: RefObject<WasmBoard | null> = useRef(null);
    if (boardRef.current === null) {
        boardRef.current = new WasmBoard();
    }

    const [cells, setCells] = useState(() => boardRef.current!.to_js());

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
                    onUpdate={onUpdate}
                    goLeft={makeGo(-1)}
                    goRight={makeGo(+1)}
                    goUp={makeGo(-9)}
                    goDown={makeGo(+9)}
                />
            )
        }
    }

    return <div className='board'>
        {children}
    </div>;
}

export default Board
