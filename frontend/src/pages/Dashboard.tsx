import { useState, useEffect } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Users, Home, CalendarDays, DollarSign, MessageCircle, BarChart3 } from 'lucide-react'
import { dashboardService, DashboardStats, DashboardActivity } from '@/services/dashboard'
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell,
  LineChart,
  Line,
  Legend
} from 'recharts'

const COLORS = ['#0088FE', '#00C49F', '#FFBB28', '#FF8042', '#8884d8']

export default function Dashboard() {
  const [stats, setStats] = useState<DashboardStats | null>(null)
  const [activity, setActivity] = useState<DashboardActivity[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const loadData = async () => {
    try {
      setLoading(true)
      setError(null)
      const [statsRes, activityRes] = await Promise.all([
        dashboardService.getStats(),
        dashboardService.getActivity(),
      ])
      // La API puede retornar null si hay redirección por 401
      if (!statsRes) throw new Error('No se pudieron cargar las métricas. Verifica tu sesión.')
      setStats(statsRes)
      setActivity(Array.isArray(activityRes) ? activityRes : [])
    } catch (err: any) {
      console.error('Error loading dashboard data:', err)
      setError(err.message || 'Error al cargar el dashboard')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadData()
  }, [])

  if (loading) {
    return (
      <div className="flex flex-col items-center justify-center p-12 text-muted-foreground gap-3">
        <div className="h-8 w-8 rounded-full border-2 border-primary border-t-transparent animate-spin" />
        <span>Cargando métricas...</span>
      </div>
    )
  }

  if (error || !stats) {
    return (
      <div className="flex flex-col items-center justify-center p-12 gap-4">
        <div className="p-4 text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20 border border-red-100 dark:border-red-800 rounded-md text-center max-w-md">
          <p className="font-semibold mb-1">Error al cargar el dashboard</p>
          <p className="text-sm">{error || 'No se recibieron datos del servidor'}</p>
        </div>
        <button
          onClick={loadData}
          className="px-4 py-2 text-sm rounded-md bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
        >
          Reintentar
        </button>
      </div>
    )
  }

  const STATS_UI = [
    { title: 'Propiedades Activas', value: stats.total_properties, icon: Home, trend: 'Activas' },
    { title: 'Nuevos Leads', value: stats.new_leads, icon: Users, trend: 'Sin contactar' },
    { title: 'Citas Futuras', value: stats.upcoming_appointments, icon: CalendarDays, trend: 'Programadas' },
    { title: 'Total Clientes', value: stats.total_clients, icon: DollarSign, trend: 'Registrados' },
    { title: 'WhatsApp Activos', value: stats.active_whatsapp_conversations, icon: MessageCircle, trend: 'Abiertas' },
    { title: 'Leads del Mes', value: stats.leads_this_month, icon: BarChart3, trend: 'En curso' },
  ]

  // Data mapping for charts
  const leadsByStatusData = stats.leads_by_status || []
  const convByAgentData = stats.conversations_by_agent || []
  const conversionsByMonth = stats.conversions_by_month || []

  // Commercial Funnel approximation
  const funnelData = [
    { name: 'Nuevos', value: stats.leads_this_month },
    { name: 'En Cita', value: stats.upcoming_appointments },
    { name: 'Cerrados/Ganados', value: stats.conversions_this_month },
  ]

  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-foreground">Hola!</h1>
          <p className="text-muted-foreground">Aquí tienes el resumen de tu actividad.</p>
        </div>
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {STATS_UI.map((stat, i) => (
          <Card key={i}>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium text-muted-foreground">
                {stat.title}
              </CardTitle>
              <stat.icon className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold text-foreground">{stat.value}</div>
              <p className="text-xs text-muted-foreground mt-1">{stat.trend}</p>
            </CardContent>
          </Card>
        ))}
      </div>

      <div className="grid gap-4 md:grid-cols-2">
        {/* Gráfico 1: Embudo Comercial (BarChart horizontal) */}
        <Card>
          <CardHeader>
            <CardTitle>Embudo Comercial (Mes Actual)</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="h-[300px]">
              <ResponsiveContainer width="100%" height="100%">
                <BarChart data={funnelData} layout="vertical" margin={{ top: 5, right: 30, left: 40, bottom: 5 }}>
                  <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" />
                  <XAxis type="number" stroke="hsl(var(--muted-foreground))" tick={{ fill: 'hsl(var(--muted-foreground))' }} />
                  <YAxis dataKey="name" type="category" stroke="hsl(var(--muted-foreground))" tick={{ fill: 'hsl(var(--muted-foreground))' }} />
                  <Tooltip 
                    contentStyle={{ backgroundColor: 'hsl(var(--card))', borderColor: 'hsl(var(--border))', color: 'hsl(var(--foreground))' }}
                    itemStyle={{ color: 'hsl(var(--foreground))' }}
                  />
                  <Bar dataKey="value" fill="#3b82f6" radius={[0, 4, 4, 0]} />
                </BarChart>
              </ResponsiveContainer>
            </div>
          </CardContent>
        </Card>

        {/* Gráfico 2: Conversión Mensual */}
        <Card>
          <CardHeader>
            <CardTitle>Conversiones Mensuales</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="h-[300px]">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={conversionsByMonth} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
                  <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" />
                  <XAxis dataKey="month" stroke="hsl(var(--muted-foreground))" tick={{ fill: 'hsl(var(--muted-foreground))' }} />
                  <YAxis stroke="hsl(var(--muted-foreground))" tick={{ fill: 'hsl(var(--muted-foreground))' }} />
                  <Tooltip 
                    contentStyle={{ backgroundColor: 'hsl(var(--card))', borderColor: 'hsl(var(--border))', color: 'hsl(var(--foreground))' }}
                    itemStyle={{ color: 'hsl(var(--foreground))' }}
                  />
                  <Line type="monotone" dataKey="count" name="Ganados" stroke="#10b981" strokeWidth={2} />
                </LineChart>
              </ResponsiveContainer>
            </div>
          </CardContent>
        </Card>

        {/* Gráfico 3: Leads por Estado */}
        <Card>
          <CardHeader>
            <CardTitle>Leads por Estado</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="h-[300px]">
              <ResponsiveContainer width="100%" height="100%">
                <PieChart>
                  <Pie
                    data={leadsByStatusData}
                    cx="50%"
                    cy="50%"
                    innerRadius={60}
                    outerRadius={80}
                    paddingAngle={5}
                    dataKey="count"
                    nameKey="status"
                    label
                  >
                    {leadsByStatusData.map((entry, index) => (
                      <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                    ))}
                  </Pie>
                  <Tooltip 
                    contentStyle={{ backgroundColor: 'hsl(var(--card))', borderColor: 'hsl(var(--border))', color: 'hsl(var(--foreground))' }}
                    itemStyle={{ color: 'hsl(var(--foreground))' }}
                  />
                  <Legend wrapperStyle={{ color: 'hsl(var(--foreground))' }} />
                </PieChart>
              </ResponsiveContainer>
            </div>
          </CardContent>
        </Card>

        {/* Gráfico 4: Conversaciones por Agente */}
        <Card>
          <CardHeader>
            <CardTitle>Conversaciones Abiertas por Agente</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="h-[300px]">
              <ResponsiveContainer width="100%" height="100%">
                <BarChart data={convByAgentData} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
                  <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" />
                  <XAxis dataKey="agent_name" stroke="hsl(var(--muted-foreground))" tick={{ fill: 'hsl(var(--muted-foreground))' }} />
                  <YAxis stroke="hsl(var(--muted-foreground))" tick={{ fill: 'hsl(var(--muted-foreground))' }} />
                  <Tooltip 
                    contentStyle={{ backgroundColor: 'hsl(var(--card))', borderColor: 'hsl(var(--border))', color: 'hsl(var(--foreground))' }}
                    itemStyle={{ color: 'hsl(var(--foreground))' }}
                  />
                  <Bar dataKey="count" fill="#8b5cf6" radius={[4, 4, 0, 0]} />
                </BarChart>
              </ResponsiveContainer>
            </div>
          </CardContent>
        </Card>
      </div>

    </div>
  )
}
