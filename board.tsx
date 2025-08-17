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
