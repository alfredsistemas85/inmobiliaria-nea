import React, { useState } from 'react';
import { X } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { clientsService } from '@/services/clients';

interface QuickClientModalProps {
  role: string;
  onClose: () => void;
  onSuccess: (clientId: string) => void;
}

export function QuickClientModal({ role, onClose, onSuccess }: QuickClientModalProps) {
  const [firstName, setFirstName] = useState('');
  const [lastName, setLastName] = useState('');
  const [documentNumber, setDocumentNumber] = useState('');
  const [email, setEmail] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!firstName || !lastName || !documentNumber) {
      setError('Nombre, apellido y documento son obligatorios');
      return;
    }
    
    try {
      setLoading(true);
      setError('');
      // Simplified client creation for the quick modal
      const payload = {
        first_name: firstName,
        last_name: lastName,
        document_type: 'DNI',
        document_number: documentNumber,
        email: email || undefined,
        c_type: 'PERSON'
      };
      
      const newClient = await clientsService.createClient(payload);
      onSuccess(newClient.id);
    } catch (err: any) {
      setError(err.message || 'Error al crear cliente');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 z-[60] flex items-center justify-center bg-black/60 p-4">
      <div className="bg-zinc-900 border border-zinc-800 rounded-xl shadow-2xl w-full max-w-md flex flex-col">
        <div className="flex justify-between items-center p-4 border-b border-zinc-800">
          <h2 className="text-lg font-semibold text-white">
            Nuevo {role === 'GUARANTOR' ? 'Garante' : role === 'TENANT' ? 'Inquilino' : 'Cliente'}
          </h2>
          <button onClick={onClose} className="text-zinc-400 hover:text-white">
            <X size={20} />
          </button>
        </div>
        <div className="p-4">
          {error && <div className="mb-4 p-2 bg-red-500/20 text-red-400 text-sm rounded">{error}</div>}
          <form onSubmit={handleSubmit} className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm text-zinc-400 mb-1">Nombre</label>
                <Input value={firstName} onChange={e => setFirstName(e.target.value)} required />
              </div>
              <div>
                <label className="block text-sm text-zinc-400 mb-1">Apellido</label>
                <Input value={lastName} onChange={e => setLastName(e.target.value)} required />
              </div>
            </div>
            <div>
              <label className="block text-sm text-zinc-400 mb-1">DNI / Documento</label>
              <Input value={documentNumber} onChange={e => setDocumentNumber(e.target.value)} required />
            </div>
            <div>
              <label className="block text-sm text-zinc-400 mb-1">Email (Opcional)</label>
              <Input type="email" value={email} onChange={e => setEmail(e.target.value)} />
            </div>
            <div className="flex justify-end gap-2 mt-6">
              <Button type="button" variant="ghost" onClick={onClose}>Cancelar</Button>
              <Button type="submit" disabled={loading}>
                {loading ? 'Guardando...' : 'Guardar y Seleccionar'}
              </Button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}
