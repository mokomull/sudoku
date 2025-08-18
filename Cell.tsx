import { JSX, KeyboardEventHandler, RefObject } from 'react';

import { Cell as WasmCell } from './pkg'

type Go = () => void;

type CellProps = {
    state: WasmCell,
    ref: RefObject<HTMLDivElement | null>,
    onUpdate: (value: number) => void,
    goLeft: Go,
    goRight: Go,
    goUp: Go,
    goDown: Go
};

export default function Cell({ state, ref, onUpdate, goLeft, goRight, goUp, goDown }: CellProps) {
    let children: JSX.Element[] = [];
    if ("Solved" in state) {
        children = [<div key="solved" className="solved">{state.Solved}</div>];
    } else {
        for (const choice of state.Choices) {
            children.push(<div key={choice} className={"choice-" + choice}>{choice}</div>)
        }
    }

    const onKeyUp: KeyboardEventHandler = function (e) {
        // Keyboard navigation mostly because bouncing between the keyboard and mouse is annoying.
        switch (e.key) {
            case "ArrowRight":
                goRight();
                e.stopPropagation();
                return;
            case "ArrowLeft":
                goLeft();
                e.stopPropagation();
                return;
            case "ArrowUp":
                goUp();
                e.stopPropagation();
                return;
            case "ArrowDown":
                goDown();
                e.stopPropagation();
                return;
        }

        // and if it isn't a navigation key, then maybe it's a number that we should use!
        const value = parseInt(e.key);
        if (isNaN(value)) {
            return;
        }
        onUpdate(value);
        e.stopPropagation();
    }

    return <div ref={ref} className="cell" tabIndex={-1} onKeyUp={onKeyUp}>{children}</div>
};
