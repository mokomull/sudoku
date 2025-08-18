import { JSX } from 'react';

import { Cell as WasmCell } from './pkg'

export default function Cell({ state }: { state: WasmCell }) {
    let children: JSX.Element[] = [];
    if ("Solved" in state) {
        children = [<div key="solved" className="solved">{state.Solved}</div>];
    } else {
        for (const choice of state.Choices) {
            children.push(<div key={choice} className={"choice-" + choice}>{choice}</div>)
        }
    }

    return <div className="cell" tabIndex={-1}>{children}</div>
};
