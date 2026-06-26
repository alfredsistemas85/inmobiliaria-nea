import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'
import { ThemeProvider } from './context/ThemeContext'

import { QueryClient, QueryClientProvider } from '@tanstack/react-query'

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 1,
      staleTime: 5 * 60 * 1000, // 5 minutos por defecto para no saturar al backend
    },
  },
})

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      {/* defaultTheme="dark" → oscuro predeterminado; storageKey persiste en localStorage */}
      <ThemeProvider defaultTheme="dark" storageKey="inmobicrm-theme">
        <App />
      </ThemeProvider>
    </QueryClientProvider>
  </React.StrictMode>,
)
