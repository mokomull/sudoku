import { RefObject, useRef, useState } from 'react'

import { Board as WasmBoard } from './pkg'
import Cell from './Cell.tsx'

function Board() {
    // this is the "Avoiding recreating the ref contents" example, but with an additional type hint
    // so that TypeScript can know what I'm *going* to put in it.
    const boardRef: RefObject<WasmBoard | null> = useRef(null);
    if (boardRef.current === null) {
        boardRef.current = new WasmBoard();
    }

    const [cells, setCells] = useState(() => boardRef.current!.to_js());

    // TODO: how do I express the type of the inner React element?  any is my escape hatch for now,
    // because it's what TS inferred before eslint told me to use the [] syntax instead of Array().
    const children: any[] = [];
    for (let x = 0; x < 9; ++x) {
        for (let y = 0; y < 9; ++y) {
            const index = x * 9 + y;

            children.push(
                // TODO: is the key *really* needed, since this list never actually changes?
                // the warning sent me to https://react.dev/learn/rendering-lists#why-does-react-need-keys
                <Cell key={index} state={cells[index]} />
            )
        }
    }

    return <div className='board'>
        {children}
    </div>;
}

export default Board
