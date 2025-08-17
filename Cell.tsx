import { Cell as WasmCell } from './pkg'

export default function Cell({ state }: { state: WasmCell }) {
    // TODO: another any just to make the red squiggle go away
    let children: any[] = [];
    if ("Solved" in state) {
        children = [<div className="solved">state.Solved</div>];
    } else {
        for (const choice of state.Choices) {
            children.push(<div className="choice-{choice}">{choice}</div>)
        }
    }

    return <div className="cell">{children}</div>
};
