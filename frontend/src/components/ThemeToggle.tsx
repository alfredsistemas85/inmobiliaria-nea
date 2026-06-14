import { Moon, Sun, Laptop } from "lucide-react"
import { useTheme } from "@/context/ThemeContext"

export function ThemeToggle() {
  const { theme, setTheme } = useTheme()

  const toggle = () => {
    if (theme === 'light') setTheme('dark')
    else if (theme === 'dark') setTheme('system')
    else setTheme('light')
  }

  return (
    <button
      onClick={toggle}
      className="relative flex h-9 w-9 items-center justify-center rounded-md border border-border bg-background text-muted-foreground hover:bg-muted hover:text-foreground transition-colors"
      title={`Tema actual: ${theme}`}
    >
      {theme === 'light' && <Sun className="h-4 w-4" />}
      {theme === 'dark' && <Moon className="h-4 w-4" />}
      {theme === 'system' && <Laptop className="h-4 w-4" />}
    </button>
  )
}
