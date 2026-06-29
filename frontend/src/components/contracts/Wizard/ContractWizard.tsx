import React, { useState } from 'react';
import { useFormContext } from 'react-hook-form';
import { Button } from '@/components/ui/button';
import { useWizardContext } from './WizardProvider';
import { fetchApi } from '@/services/api';
import { logger } from '@/lib/logger';
import { PropertySummary } from './components/PropertySummary';
import { BasicDataStep } from './steps/BasicDataStep';
import { ParticipantsStep } from './steps/ParticipantsStep';
import { GuarantorsStep } from './steps/GuarantorsStep';
import { TermsStep } from './steps/TermsStep';
import { ClausesStep } from './steps/ClausesStep';

interface ContractWizardProps {
  onCancel: () => void;
  onSuccess: () => void;
}

export function ContractWizard({ onCancel, onSuccess }: ContractWizardProps) {
  const { 
    step, globalError, setGlobalError, handleNext, handleBack, 
    draftAvailable, handleRestoreDraft, handleDiscardDraft, clearDraft 
  } = useWizardContext();
  
  const { handleSubmit, formState: { isSubmitting } } = useFormContext<any>();
  const [isSaving, setIsSaving] = useState(false);

  const onSubmit = async (data: any) => {
    try {
      setGlobalError('');
      setIsSaving(true);
      
      const payload = {
        ...data,
        status: 'DRAFT',
        commission_amount: 0,
        fees_amount: 0,
        clauses: data.clauses.map((c: any, i: number) => ({ ...c, display_order: i + 1 })),
        template_id: data.template_id || null
      };

      await fetchApi('/contracts/v2', {
        method: 'POST',
        body: JSON.stringify(payload)
      });
      
      logger.info('Wizard', 'contract_created');
      clearDraft();
      onSuccess();
    } catch (err: any) {
      logger.error('Wizard', 'contract_creation_failed', { error: err.message, status: err.status });
      
      // Manejo específico de errores
      if (err.status === 401) {
        setGlobalError('Sesión expirada.');
      } else if (err.status === 403) {
        setGlobalError('No posee permisos.');
      } else if (err.status === 409) {
        setGlobalError('Ya existe un contrato activo para esas fechas.');
      } else if (err.status === 422) {
        setGlobalError('Los datos enviados no son válidos.');
      } else if (err.status >= 500 || err.status === 0) {
        setGlobalError(err.status === 0 ? 'El servidor no respondió.' : `Error interno: ${err.message}`);
      } else {
        setGlobalError(err.message || 'Error al crear el contrato');
      }
    } finally {
      setIsSaving(false);
    }
  };

  const currentStep = () => {
    switch (step) {
      case 1: return <BasicDataStep />;
      case 2: return <ParticipantsStep />;
      case 3: return <GuarantorsStep />;
      case 4: return <TermsStep />;
      case 5: return <ClausesStep />;
      default: return null;
    }
  };

  const totalSteps = 5;

  if (draftAvailable) {
    return (
      <div className="flex flex-col items-center justify-center h-64 space-y-6 text-center">
        <div className="text-xl font-medium text-white">Se encontró un borrador anterior.</div>
        <div className="text-zinc-400">¿Desea restaurarlo?</div>
        <div className="flex space-x-4">
          <Button variant="outline" onClick={handleDiscardDraft}>Descartar</Button>
          <Button onClick={handleRestoreDraft}>Restaurar</Button>
        </div>
      </div>
    );
  }

  return (
    <div className="w-full flex flex-col h-full">
      <div className="flex-1">
        {globalError && <div className="mb-4 p-3 bg-red-500/20 text-red-400 rounded-lg">{globalError}</div>}
        
        <div className="flex mb-6 space-x-2">
          {Array.from({ length: totalSteps }).map((_, i) => (
            <div key={i} className={`h-2 flex-1 rounded-full ${step >= i + 1 ? 'bg-cyan-500' : 'bg-zinc-800'}`} />
          ))}
        </div>

        <PropertySummary />

        <form onSubmit={e => e.preventDefault()}>
          {currentStep()}
        </form>
      </div>

      <div className="mt-8 pt-4 border-t border-zinc-800 flex justify-between">
        <Button type="button" variant="ghost" onClick={step > 1 ? handleBack : onCancel}>
          {step > 1 ? 'Atrás' : 'Cancelar'}
        </Button>
        
        {step < totalSteps ? (
          <Button type="button" onClick={handleNext}>Siguiente</Button>
        ) : (
          <Button type="button" onClick={handleSubmit(onSubmit)} disabled={isSubmitting || isSaving}>
            {isSubmitting || isSaving ? 'Creando...' : 'Finalizar y Crear Contrato'}
          </Button>
        )}
      </div>
    </div>
  );
}
