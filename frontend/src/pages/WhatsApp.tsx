import { useState } from 'react'
import { Search, Send, Paperclip, MoreVertical, CheckCheck } from 'lucide-react'
import { Input } from '@/components/ui/input'

const MOCK_CHATS = [
  { id: 1, name: 'Esteban Quito', lastMessage: '¿Sigue disponible la casa?', time: '10:45 AM', unread: 2, online: true },
  { id: 2, name: 'María Gómez', lastMessage: 'Perfecto, nos vemos el martes.', time: 'Ayer', unread: 0, online: false },
  { id: 3, name: 'Carlos Mendoza', lastMessage: 'Envié los documentos requeridos.', time: 'Lunes', unread: 0, online: true },
  { id: 4, name: 'Ana Silva', lastMessage: 'Gracias por la información.', time: '12 May', unread: 0, online: false },
]

export default function WhatsApp() {
  const [activeChat, setActiveChat] = useState(MOCK_CHATS[0])

  return (
    <div className="h-[calc(100vh-8rem)] flex flex-col md:flex-row bg-white rounded-xl border border-slate-200 overflow-hidden shadow-sm">
      {/* Sidebar de Chats */}
      <div className="w-full md:w-80 border-r border-slate-200 flex flex-col h-full bg-slate-50">
        <div className="p-4 border-b border-slate-200 bg-white">
          <h2 className="text-lg font-semibold text-slate-900 mb-4">Mensajes</h2>
          <div className="relative">
            <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-slate-400" />
            <Input 
              placeholder="Buscar chat..." 
              className="pl-9 bg-slate-50 border-slate-200 h-9"
            />
          </div>
        </div>
        
        <div className="flex-1 overflow-y-auto">
          {MOCK_CHATS.map((chat) => (
            <div 
              key={chat.id}
              onClick={() => setActiveChat(chat)}
              className={`p-3 flex gap-3 cursor-pointer border-b border-slate-100 transition-colors ${
                activeChat.id === chat.id ? 'bg-blue-50/50' : 'hover:bg-white'
              }`}
            >
              <div className="relative shrink-0">
                <div className="h-12 w-12 rounded-full bg-slate-200 flex items-center justify-center text-slate-600 font-medium text-lg">
                  {chat.name.charAt(0)}
                </div>
                {chat.online && (
                  <span className="absolute bottom-0 right-0 h-3 w-3 rounded-full bg-green-500 border-2 border-white"></span>
                )}
              </div>
              
              <div className="flex-1 min-w-0">
                <div className="flex justify-between items-baseline mb-1">
                  <h3 className="font-medium text-slate-900 truncate pr-2">{chat.name}</h3>
                  <span className="text-xs text-slate-500 shrink-0">{chat.time}</span>
                </div>
                <div className="flex justify-between items-center">
                  <p className="text-sm text-slate-500 truncate pr-2">{chat.lastMessage}</p>
                  {chat.unread > 0 && (
                    <span className="bg-green-500 text-white text-[10px] font-bold px-2 py-0.5 rounded-full shrink-0">
                      {chat.unread}
                    </span>
                  )}
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Área Principal de Chat */}
      <div className="flex-1 flex flex-col h-full bg-slate-50/50 hidden md:flex">
        {/* Chat Header */}
        <div className="h-16 border-b border-slate-200 bg-white flex items-center justify-between px-6">
          <div className="flex items-center gap-3">
            <div className="h-10 w-10 rounded-full bg-slate-200 flex items-center justify-center text-slate-600 font-medium">
              {activeChat.name.charAt(0)}
            </div>
            <div>
              <h3 className="font-medium text-slate-900">{activeChat.name}</h3>
              <p className="text-xs text-green-500">{activeChat.online ? 'En línea' : 'Desconectado'}</p>
            </div>
          </div>
          <div className="flex items-center gap-2 text-slate-400">
            <button className="p-2 hover:bg-slate-100 rounded-full transition-colors"><Search className="h-5 w-5" /></button>
            <button className="p-2 hover:bg-slate-100 rounded-full transition-colors"><MoreVertical className="h-5 w-5" /></button>
          </div>
        </div>

        {/* Mensajes del Chat */}
        <div className="flex-1 overflow-y-auto p-6 space-y-4">
          <div className="flex justify-center">
            <span className="bg-slate-200/50 text-slate-500 text-xs px-3 py-1 rounded-full">Hoy</span>
          </div>
          
          <div className="flex gap-3 max-w-[80%]">
            <div className="h-8 w-8 rounded-full bg-slate-200 shrink-0 flex items-center justify-center text-slate-600 text-xs">
              {activeChat.name.charAt(0)}
            </div>
            <div className="bg-white border border-slate-200 rounded-2xl rounded-tl-sm p-3 shadow-sm">
              <p className="text-sm text-slate-800">Hola, vi la publicación de la casa en San Isidro.</p>
              <span className="text-[10px] text-slate-400 mt-1 block">10:44 AM</span>
            </div>
          </div>

          <div className="flex gap-3 max-w-[80%]">
            <div className="h-8 w-8 rounded-full bg-slate-200 shrink-0 flex items-center justify-center text-slate-600 text-xs opacity-0">
              {activeChat.name.charAt(0)}
            </div>
            <div className="bg-white border border-slate-200 rounded-2xl rounded-tl-sm p-3 shadow-sm">
              <p className="text-sm text-slate-800">¿Sigue disponible la casa?</p>
              <span className="text-[10px] text-slate-400 mt-1 block">10:45 AM</span>
            </div>
          </div>

          <div className="flex gap-3 max-w-[80%] ml-auto flex-row-reverse">
            <div className="bg-green-600 text-white rounded-2xl rounded-tr-sm p-3 shadow-sm">
              <p className="text-sm">Hola {activeChat.name.split(' ')[0]}! Sí, la propiedad sigue disponible.</p>
              <div className="flex items-center justify-end gap-1 mt-1">
                <span className="text-[10px] text-green-200">10:48 AM</span>
                <CheckCheck className="h-3 w-3 text-white" />
              </div>
            </div>
          </div>
          
          <div className="flex gap-3 max-w-[80%] ml-auto flex-row-reverse">
            <div className="bg-green-600 text-white rounded-2xl rounded-tr-sm p-3 shadow-sm">
              <p className="text-sm">¿Te gustaría coordinar una visita para conocerla?</p>
              <div className="flex items-center justify-end gap-1 mt-1">
                <span className="text-[10px] text-green-200">10:49 AM</span>
                <CheckCheck className="h-3 w-3 text-white" />
              </div>
            </div>
          </div>
        </div>

        {/* Área de Input */}
        <div className="h-20 bg-white border-t border-slate-200 p-4 flex items-center gap-3">
          <button className="text-slate-400 hover:text-slate-600 p-2"><Paperclip className="h-5 w-5" /></button>
          <div className="flex-1 relative">
            <input 
              type="text" 
              placeholder="Escribe un mensaje..." 
              className="w-full bg-slate-100 border-transparent rounded-full pl-4 pr-10 py-3 text-sm focus:bg-white focus:border-slate-300 focus:ring-2 focus:ring-blue-100 outline-none transition-all"
            />
          </div>
          <button className="bg-blue-600 hover:bg-blue-700 text-white p-3 rounded-full transition-colors flex items-center justify-center">
            <Send className="h-5 w-5 ml-1" />
          </button>
        </div>
      </div>
    </div>
  )
}
