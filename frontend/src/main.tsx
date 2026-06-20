import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'
import { ThemeProvider } from './context/ThemeContext'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    {/* defaultTheme="dark" → oscuro predeterminado; storageKey persiste en localStorage */}
    <ThemeProvider defaultTheme="dark" storageKey="inmobicrm-theme">
      <App />
    </ThemeProvider>
  </React.StrictMode>,
)
