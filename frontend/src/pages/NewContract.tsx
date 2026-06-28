import React from 'react';
import { useSearchParams, useNavigate } from 'react-router-dom';
import { WizardProvider } from '@/components/contracts/Wizard/WizardProvider';
import { ContractWizard } from '@/components/contracts/Wizard/ContractWizard';
import { ArrowLeft } from 'lucide-react';
import { Button } from '@/components/ui/button';

import { ContractWizardErrorBoundary } from '@/components/contracts/Wizard/ContractWizardErrorBoundary';

export default function NewContract() {
  const [searchParams] = useSearchParams();
  const propertyId = searchParams.get('property_id');
  const navigate = useNavigate();
  // Forzamos remount si hay un reset desde el error boundary
  const [resetKey, setResetKey] = React.useState(0);

  const handleCancel = () => {
    navigate('/contracts');
  };

  const handleSuccess = () => {
    navigate('/contracts');
  };

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div className="flex items-center gap-4">
        <Button variant="ghost" size="icon" className="rounded-full" onClick={handleCancel}>
          <ArrowLeft className="h-5 w-5" />
        </Button>
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-foreground">Nuevo Contrato</h1>
          <p className="text-muted-foreground">Completá los datos para generar el contrato de alquiler.</p>
        </div>
      </div>

      <div className="bg-zinc-900 border border-zinc-800 rounded-xl shadow-sm p-6">
        <ContractWizardErrorBoundary 
          key={resetKey}
          onReset={() => setResetKey(k => k + 1)} 
          onBackToDashboard={handleCancel}
        >
          <WizardProvider initialPropertyId={propertyId}>
            <ContractWizard onCancel={handleCancel} onSuccess={handleSuccess} />
          </WizardProvider>
        </ContractWizardErrorBoundary>
      </div>
    </div>
  );
}
