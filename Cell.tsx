import { Context, createContext, JSX, KeyboardEventHandler, RefObject, useContext } from 'react';

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

export const HighlightContext = createContext(null as string | null);

export default function Cell({ state, ref, onUpdate, goLeft, goRight, goUp, goDown }: CellProps) {
    let children: JSX.Element[] = [];
    const highlight = useContext(HighlightContext);
    var highlightWholeCellClass = "";
    if ("Solved" in state) {
        highlightWholeCellClass = (highlight == state.Solved) ? " highlighted" : "";
        children = [<div key="solved" className="solved">{state.Solved}</div>];
    } else {
        for (const choice of state.Choices) {
            const highlightClass = (highlight == choice) ? " highlighted" : "";
            children.push(<div key={choice} className={"choice-" + choice + highlightClass}>{choice}</div>)
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

    return <div ref={ref} className={"cell" + highlightWholeCellClass} tabIndex={-1} onKeyUp={onKeyUp}>{children}</div>
};
