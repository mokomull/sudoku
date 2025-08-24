import { Hint as WasmHint } from './pkg';

type HintProps = {
    hint: WasmHint,
    onEnter: () => void,
    onLeave: () => void,
    commit: () => void,
};

export default function ({ hint, onEnter, onLeave, commit }: HintProps) {
    return <div
        className="hint"
        onMouseEnter={onEnter}
        onMouseLeave={onLeave}
        onClick={commit}>
        {hint.description}
    </div>
};
