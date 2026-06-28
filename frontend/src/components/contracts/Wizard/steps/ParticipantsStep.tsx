import React, { useState, useEffect } from 'react';
import { useFormContext, useFieldArray } from 'react-hook-form';
import { Plus } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { ContractFormValues } from '../schema';
import { ParticipantSelector } from '../components/ParticipantSelector';
import { QuickClientModal } from '../components/QuickClientModal';
import { useWizardContext } from '../WizardProvider';
import { clientsService, Client } from '@/services/clients';

export const ParticipantsStep = React.memo(function ParticipantsStep() {
  const { control } = useFormContext<ContractFormValues>();
  const { isPropertyCentric } = useWizardContext();
  const [clients, setClients] = useState<Client[]>([]);
  const [quickClientRole, setQuickClientRole] = useState<string | null>(null);

  const { fields, append, remove } = useFieldArray({
    control,
    name: "participants"
  });

  useEffect(() => {
    clientsService.getClients(100).then(res => setClients(res.data || []));
  }, []);

  const addParticipant = (role: string, is_main: boolean) => {
    append({
      client_id: '',
      p_role: role,
      percentage: 100,
      is_main,
      guarantees: []
    });
  };

  const handleQuickClientSuccess = async (clientId: string) => {
    const res = await clientsService.getClients(100);
    setClients(res.data || []);
    setQuickClientRole(null);
    
    if (quickClientRole) {
      append({
        client_id: clientId,
        p_role: quickClientRole,
        percentage: 100,
        is_main: true, // simplified
        guarantees: []
      });
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center mb-6">
        <h3 className="text-lg text-white font-medium">Paso 2: Locadores y Locatarios</h3>
        <div className="space-x-2">
          {!isPropertyCentric && (
            <Button type="button" onClick={() => addParticipant('LANDLORD', true)} variant="outline" size="sm">
              <Plus size={16} className="mr-1"/> Locador
            </Button>
          )}
          <Button type="button" onClick={() => addParticipant('TENANT', true)} variant="outline" size="sm">
            <Plus size={16} className="mr-1"/> Locatario
          </Button>
        </div>
      </div>
      
      {fields.map((field, i) => (
        <ParticipantSelector 
          key={field.id} 
          index={i} 
          clients={clients} 
          isPropertyCentric={isPropertyCentric} 
          onRemove={remove}
          onQuickClient={(role) => setQuickClientRole(role)}
        />
      ))}

      {quickClientRole && (
        <QuickClientModal 
          role={quickClientRole} 
          onClose={() => setQuickClientRole(null)} 
          onSuccess={handleQuickClientSuccess} 
        />
      )}
    </div>
  );
}
