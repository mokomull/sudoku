import { JSX } from 'react';

import { Cell as WasmCell } from './pkg'

export default function Cell({ state, onUpdate }: { state: WasmCell, onUpdate: (value: number) => void }) {
    let children: JSX.Element[] = [];
    if ("Solved" in state) {
        children = [<div key="solved" className="solved">{state.Solved}</div>];
    } else {
        for (const choice of state.Choices) {
            children.push(<div key={choice} className={"choice-" + choice}>{choice}</div>)
        }
    }

    const onKeyUp = function(e) {
        const value = parseInt(e.key);
        if (isNaN(value)) {
            return;
        }
        onUpdate(value);
    }

    return <div className="cell" tabIndex={-1} onKeyUp={onKeyUp}>{children}</div>
};
