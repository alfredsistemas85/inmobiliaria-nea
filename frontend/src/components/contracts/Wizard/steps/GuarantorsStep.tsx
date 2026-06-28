import React, { useState, useEffect } from 'react';
import { useFormContext, useFieldArray } from 'react-hook-form';
import { Plus, Trash2 } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { ContractFormValues } from '../schema';
import { QuickClientModal } from '../components/QuickClientModal';
import { clientsService, Client } from '@/services/clients';

export const GuarantorsStep = React.memo(function GuarantorsStep() {
  const { control, register, watch, setValue, formState: { errors } } = useFormContext<ContractFormValues>();
  const [clients, setClients] = useState<Client[]>([]);
  const [showQuickClient, setShowQuickClient] = useState(false);

  const { fields, append, remove } = useFieldArray({
    control,
    name: "participants"
  });

  useEffect(() => {
    clientsService.getClients(100).then(res => setClients(res.data || []));
  }, []);

  const addGuarantor = () => {
    append({
      client_id: '',
      p_role: 'GUARANTOR',
      percentage: 100,
      is_main: false,
      guarantees: []
    });
  };

  const addGuarantee = (pIndex: number) => {
    const parts = [...watch('participants')];
    if (!parts[pIndex].guarantees) parts[pIndex].guarantees = [];
    parts[pIndex].guarantees.push({
      guarantee_type: 'PAYSLIP',
      income_amount: 0,
      employer: '',
      guarantee_details: ''
    });
    setValue('participants', parts);
  };

  const removeGuarantee = (pIndex: number, gIndex: number) => {
    const parts = [...watch('participants')];
    parts[pIndex].guarantees.splice(gIndex, 1);
    setValue('participants', parts);
  };

  const handleQuickClientSuccess = async (clientId: string) => {
    const res = await clientsService.getClients(100);
    setClients(res.data || []);
    setShowQuickClient(false);
    
    append({
      client_id: clientId,
      p_role: 'GUARANTOR',
      percentage: 100,
      is_main: false,
      guarantees: []
    });
  };

  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center mb-2">
        <h3 className="text-lg text-white font-medium">Paso 3: Garantes</h3>
        <Button type="button" onClick={addGuarantor} variant="outline" size="sm">
          <Plus size={16} className="mr-1"/> Agregar Garante
        </Button>
      </div>
      
      <div className="text-sm text-zinc-400 mb-6">Los garantes respaldan las obligaciones del locatario.</div>

      {fields.map((field, i) => {
        if (watch(`participants.${i}.p_role`) !== 'GUARANTOR') return null;
        
        return (
          <Card key={field.id} className="bg-zinc-800 border-zinc-700 mb-4">
            <CardHeader className="p-4 pb-0 flex flex-row justify-between items-center">
              <div className="font-medium text-white">Garante</div>
              <div className="space-x-2">
                <Button type="button" onClick={() => addGuarantee(i)} size="sm" variant="outline">
                  <Plus size={16} className="mr-1"/> Añadir Respaldo
                </Button>
                <Button type="button" onClick={() => remove(i)} size="sm" variant="ghost" className="text-red-400">
                  <Trash2 size={16} />
                </Button>
              </div>
            </CardHeader>
            <CardContent className="p-4 space-y-4">
              <div className="flex gap-2">
                <div className="flex-1">
                  <label className="block text-sm text-zinc-400 mb-1">Cliente Garante</label>
                  <select
                    className={`w-full bg-zinc-900 border ${errors.participants?.[i]?.client_id ? 'border-red-500' : 'border-zinc-700'} rounded-lg px-4 py-2 text-white`}
                    {...register(`participants.${i}.client_id`)}
                  >
                    <option value="">Seleccionar garante...</option>
                    {clients.map(c => (
                      <option key={c.id} value={c.id}>{c.first_name} {c.last_name}</option>
                    ))}
                  </select>
                  {errors.participants?.[i]?.client_id && <span className="text-xs text-red-500">{errors.participants?.[i]?.client_id?.message}</span>}
                </div>
                <div className="flex items-end">
                  <Button type="button" variant="outline" size="icon" onClick={() => setShowQuickClient(true)} title="Nuevo Garante">
                    <Plus size={16} />
                  </Button>
                </div>
              </div>

              {watch(`participants.${i}.guarantees`)?.map((_, gIndex) => (
                <div key={gIndex} className="grid grid-cols-3 gap-4 p-3 bg-zinc-900 rounded-lg border border-zinc-700">
                  <div>
                    <label className="block text-sm text-zinc-400 mb-1">Tipo de Respaldo</label>
                    <select
                      className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-2 text-white"
                      {...register(`participants.${i}.guarantees.${gIndex}.guarantee_type`)}
                    >
                      <option value="PAYSLIP">Recibo de Sueldo</option>
                      <option value="PROPERTY">Propietario</option>
                      <option value="SURETY_BOND">Seguro de Caución</option>
                    </select>
                  </div>
                  <div className="col-span-2 flex gap-2 items-end">
                    <div className="flex-1">
                      <label className="block text-sm text-zinc-400 mb-1">Detalle / Empleador</label>
                      <Input 
                        {...register(`participants.${i}.guarantees.${gIndex}.employer`)}
                      />
                    </div>
                    <Button type="button" variant="ghost" onClick={() => removeGuarantee(i, gIndex)} className="text-red-400">
                      <Trash2 size={16} />
                    </Button>
                  </div>
                </div>
              ))}
            </CardContent>
          </Card>
        );
      })}

      {showQuickClient && (
        <QuickClientModal 
          role="GUARANTOR" 
          onClose={() => setShowQuickClient(false)} 
          onSuccess={handleQuickClientSuccess} 
        />
      )}
    </div>
  );
});
