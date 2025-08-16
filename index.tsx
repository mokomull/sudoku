import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import Board from './board.tsx'

createRoot(document.getElementById("root")!).render(<StrictMode><Board /></StrictMode>)
