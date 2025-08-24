import { Hint as WasmHint } from './pkg';

export default function ({ hint, onEnter, onLeave }: { hint: WasmHint, onEnter: () => void, onLeave: () => void }) {
    return <div className="hint" onMouseEnter={onEnter} onMouseLeave={onLeave}>{hint.description}</div>
};
