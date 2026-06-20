
import { Outlet, Link, useLocation } from 'react-router-dom'
import {
  Building2, Activity, LifeBuoy, Settings, LogOut, Search, ShieldAlert, Moon, Sun
} from 'lucide-react'
import { cn } from '@/lib/utils'
import { useTheme } from '@/context/ThemeContext'

const NAV_ITEMS = [
  { name: 'Dashboard',    path: '/superadmin',           icon: ShieldAlert, exact: true },
  { name: 'Inmobiliarias',path: '/superadmin/tenants',   icon: Building2 },
  { name: 'Monitoreo',    path: '/superadmin/monitoring',icon: Activity },
  { name: 'Soporte',      path: '/superadmin/support',   icon: LifeBuoy },
  { name: 'Ajustes',      path: '/superadmin/settings',  icon: Settings },
]

export function SuperAdminLayout() {
  const location = useLocation()
  
  const userStr = localStorage.getItem('user')
  const user = userStr ? JSON.parse(userStr) : null

  const { theme, setTheme } = useTheme()
  const isDark = theme === 'dark'

  const toggleTheme = () => {
    setTheme(isDark ? 'light' : 'dark')
  }

  return (
    <div className="flex h-screen bg-background text-foreground transition-colors duration-300">
      <aside className="hidden w-64 flex-col border-r border-border bg-card md:flex">
        <div className="flex h-16 items-center px-6 border-b border-border">
          <div className="flex items-center gap-2 font-bold text-xl text-purple-600">
            <ShieldAlert className="h-6 w-6" />
            <span>SuperAdmin</span>
          </div>
        </div>

        <div className="flex-1 overflow-y-auto py-4">
          <nav className="space-y-1 px-3">
            {NAV_ITEMS.map((item) => {
              const isActive = item.exact 
                ? location.pathname === item.path 
                : location.pathname.startsWith(item.path)

              return (
                <Link
                  key={item.name}
                  to={item.path}
                  className={cn(
                    'flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors',
                    isActive
                      ? 'bg-purple-600/10 text-purple-600'
                      : 'text-muted-foreground hover:bg-muted hover:text-foreground'
                  )}
                >
                  <item.icon
                    className={cn('h-5 w-5', isActive ? 'text-purple-600' : 'text-muted-foreground')}
                  />
                  {item.name}
                </Link>
              )
            })}
          </nav>
        </div>

        <div className="p-4 border-t border-border">
          <div className="flex items-center gap-3">
            <div className="h-9 w-9 rounded-full bg-purple-600/10 flex items-center justify-center text-purple-600 font-bold">
              SA
            </div>
            <div className="flex-1 overflow-hidden">
              <p className="truncate text-sm font-medium text-foreground">{user?.first_name || 'Super'} {user?.last_name || 'Admin'}</p>
              <p className="truncate text-xs text-muted-foreground">System Owner</p>
            </div>
            <button
              className="text-muted-foreground hover:text-foreground transition-colors"
              title="Cerrar sesión"
              onClick={() => {
                localStorage.clear()
                window.location.replace('/login')
              }}
            >
              <LogOut className="h-5 w-5" />
            </button>
          </div>
        </div>
      </aside>

      <div className="flex flex-1 flex-col overflow-hidden">
        <header className="flex h-16 items-center justify-between border-b border-border bg-card px-4 md:px-6">
          <div className="flex items-center gap-4 md:hidden">
            <div className="flex items-center gap-2 font-bold text-lg text-purple-600">
              <ShieldAlert className="h-5 w-5" />
              <span>SuperAdmin</span>
            </div>
          </div>

          <div className="hidden md:flex flex-1 items-center max-w-md">
            <div className="relative w-full">
              <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
              <input
                type="text"
                placeholder="Buscar tenants, errores..."
                className="w-full rounded-md border border-input bg-background pl-9 pr-4 py-2 text-sm outline-none focus:border-purple-600 focus:ring-1 focus:ring-purple-600 text-foreground"
              />
            </div>
          </div>

          <div className="flex items-center gap-3 ml-auto">
            <button
              onClick={toggleTheme}
              className="relative flex h-9 w-9 items-center justify-center rounded-md border border-border bg-background text-muted-foreground hover:bg-muted hover:text-foreground transition-colors duration-200"
            >
              <Sun
                className={cn(
                  'absolute h-4 w-4 transition-all duration-300',
                  isDark ? 'opacity-100 rotate-0 scale-100' : 'opacity-0 rotate-90 scale-75'
                )}
              />
              <Moon
                className={cn(
                  'absolute h-4 w-4 transition-all duration-300',
                  isDark ? 'opacity-0 -rotate-90 scale-75' : 'opacity-100 rotate-0 scale-100'
                )}
              />
            </button>
          </div>
        </header>

        <main className="flex-1 overflow-y-auto bg-background p-4 md:p-6">
          <Outlet />
        </main>
      </div>
    </div>
  )
}
