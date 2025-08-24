import { Context, createContext, JSX, KeyboardEventHandler, RefObject, useContext } from 'react';

import { DigitLocation, Cell as WasmCell } from './pkg'

type Go = () => void;

type CellProps = {
    state: WasmCell,
    ref: RefObject<HTMLDivElement | null>,
    cause: number[] | null,
    effect: number[] | null,
    onUpdate: (value: number) => void,
    goLeft: Go,
    goRight: Go,
    goUp: Go,
    goDown: Go
};

export const HighlightContext = createContext(null as string | null);

export default function Cell({ state, ref, cause, effect, onUpdate, goLeft, goRight, goUp, goDown }: CellProps) {
    let children: JSX.Element[] = [];
    const highlight = useContext(HighlightContext);
    var highlightWholeCellClass = "";
    if ("Solved" in state) {
        highlightWholeCellClass = (highlight == state.Solved) ? " highlighted" : "";
        children = [<div key="solved" className="solved">{state.Solved}</div>];
    } else {
        for (const choice of state.Choices) {
            let highlightClass = "";
            if (cause !== null && cause.includes(parseInt(choice))) {
                highlightClass = " cause";
            } else if (effect !== null && effect.includes(parseInt(choice))) {
                highlightClass = " highlighted";
            } else if (highlight == choice) {
                highlightClass = " highlighted";
            }
            children.push(<div key={choice} className={"choice-" + choice + highlightClass}>{choice}</div>)
        }
    }

    // empty arrays of digits mean the whole cell
    if (cause !== null && cause.length == 0) {
        highlightWholeCellClass = " cause";
    } else if (effect !== null && effect.length == 0) {
        highlightWholeCellClass = " effect";
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
