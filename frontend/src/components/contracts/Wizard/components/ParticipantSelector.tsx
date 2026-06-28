import React, { useState } from 'react';
import { useFormContext } from 'react-hook-form';
import { Plus, Trash2 } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent } from '@/components/ui/card';
import { ContractFormValues } from '../schema';
import { Client } from '@/services/clients';

interface ParticipantSelectorProps {
  index: number;
  clients: Client[];
  isPropertyCentric: boolean;
  onRemove: (index: number) => void;
  onQuickClient?: (role: string) => void;
}

export function ParticipantSelector({ index, clients, isPropertyCentric, onRemove, onQuickClient }: ParticipantSelectorProps) {
  const { register, watch, formState: { errors } } = useFormContext<ContractFormValues>();
  
  const pRole = watch(`participants.${index}.p_role`);
  const isLandlord = pRole === 'LANDLORD';
  const isLocked = isPropertyCentric && isLandlord;

  if (pRole === 'GUARANTOR') return null; // Guarantors have their own step

  return (
    <Card className={`bg-zinc-800 ${isLandlord ? 'border-cyan-700/50' : 'border-zinc-700'}`}>
      <CardContent className="p-4 flex gap-4 items-end">
        <div className="flex-1">
          <label className="block text-sm text-zinc-400 mb-1">
            {isLandlord ? 'Propietario / Locador' : `Cliente (${pRole})`}
          </label>
          {isLocked ? (
            <div className="w-full bg-zinc-900/50 border border-zinc-700/50 rounded-lg px-4 py-2 text-zinc-300 flex items-center justify-between">
              <span>
                {(() => {
                  const clientId = watch(`participants.${index}.client_id`);
                  const c = clients.find(c => c.id === clientId);
                  return c ? `${c.first_name} ${c.last_name}` : 'Cargando...';
                })()}
              </span>
              <span className="text-xs text-cyan-500 font-medium">Desde Propiedad</span>
            </div>
          ) : (
            <div className="flex gap-2">
              <div className="flex-1">
                <select
                  className={`w-full bg-zinc-900 border ${errors.participants?.[index]?.client_id ? 'border-red-500' : 'border-zinc-700'} rounded-lg px-4 py-2 text-white`}
                  {...register(`participants.${index}.client_id`)}
                >
                  <option value="">Seleccionar cliente...</option>
                  {clients.map(c => (
                    <option key={c.id} value={c.id}>{c.first_name} {c.last_name}</option>
                  ))}
                </select>
                {errors.participants?.[index]?.client_id && <span className="text-xs text-red-500">{errors.participants?.[index]?.client_id?.message}</span>}
              </div>
              {onQuickClient && (
                <Button type="button" variant="outline" size="icon" onClick={() => onQuickClient(pRole)} title="Nuevo Cliente Rápido">
                  <Plus size={16} />
                </Button>
              )}
            </div>
          )}
        </div>
        <div>
          <label className="block text-sm text-zinc-400 mb-1">% Part.</label>
          {isLocked ? (
            <div className="w-24 bg-zinc-900/50 border border-zinc-700/50 rounded-lg px-4 py-2 text-zinc-300 text-center">
              {watch(`participants.${index}.percentage`)}
            </div>
          ) : (
            <Input type="number" {...register(`participants.${index}.percentage`)} className="w-24" />
          )}
        </div>
        {!isLocked && (
          <Button type="button" variant="ghost" onClick={() => onRemove(index)} className="text-red-400 hover:text-red-300">
            <Trash2 size={20} />
          </Button>
        )}
      </CardContent>
    </Card>
  );
}
