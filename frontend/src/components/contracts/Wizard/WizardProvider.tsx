import React, { createContext, useContext, ReactNode } from 'react';
import { FormProvider } from 'react-hook-form';
import { useContractWizard } from './hooks/useContractWizard';

interface WizardContextType {
  step: number;
  setStep: (step: number) => void;
  globalError: string;
  setGlobalError: (error: string) => void;
  handleNext: () => Promise<void>;
  handleBack: () => void;
  loadingInitial: boolean;
  isPropertyCentric: boolean;
  property: any;
  draftAvailable: boolean;
  handleRestoreDraft: () => void;
  handleDiscardDraft: () => void;
  clearDraft: () => void;
}

const WizardContext = createContext<WizardContextType | null>(null);

export const useWizardContext = () => {
  const ctx = useContext(WizardContext);
  if (!ctx) throw new Error("useWizardContext must be used within a WizardProvider");
  return ctx;
};

interface WizardProviderProps {
  children: ReactNode;
  initialPropertyId?: string | null;
}

export function WizardProvider({ children, initialPropertyId }: WizardProviderProps) {
  const wizardState = useContractWizard(initialPropertyId);

  if (wizardState.loadingInitial) {
    return (
      <div className="flex justify-center items-center h-48">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-cyan-500"></div>
      </div>
    );
  }

  // Extraer methods para FormProvider y el resto para WizardContext
  const { methods, ...contextValue } = wizardState;

  return (
    <WizardContext.Provider value={contextValue}>
      <FormProvider {...methods}>
        {children}
      </FormProvider>
    </WizardContext.Provider>
  );
}
