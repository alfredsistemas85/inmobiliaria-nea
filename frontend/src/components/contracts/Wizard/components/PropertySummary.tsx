import React from 'react';
import { useWizardContext } from '../WizardProvider';
import { MapPin, CheckCircle2, Home } from 'lucide-react';

export function PropertySummary() {
  const { property, isPropertyCentric } = useWizardContext();

  if (!isPropertyCentric || !property) return null;

  return (
    <div className="bg-cyan-950/30 border border-cyan-800/50 rounded-xl p-4 mb-6 flex flex-col md:flex-row gap-4 justify-between items-start md:items-center">
      <div>
        <h4 className="text-cyan-400 font-medium flex items-center gap-2 mb-1">
          <Home size={16} /> {property.title}
        </h4>
        <div className="flex flex-wrap items-center gap-3 text-sm text-zinc-300">
          <span className="flex items-center gap-1"><MapPin size={14} className="text-zinc-500" /> {property.location || 'Sin ubicación'}</span>
          <span className="flex items-center gap-1"><CheckCircle2 size={14} className="text-zinc-500" /> {property.status}</span>
        </div>
      </div>
      <div className="flex gap-4 bg-zinc-900/50 p-3 rounded-lg border border-zinc-800">
        <div>
          <div className="text-xs text-zinc-500 mb-0.5">Precio Inicial</div>
          <div className="font-semibold text-white">{property.currency || 'ARS'} {Number(property.price || 0).toLocaleString()}</div>
        </div>
        <div>
          <div className="text-xs text-zinc-500 mb-0.5">Tipo</div>
          <div className="font-semibold text-white">{property.type || '-'}</div>
        </div>
      </div>
    </div>
  );
}
