import { useState, useRef, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import SignatureCanvas from 'react-signature-canvas';

export default function ContractSignaturePage() {
    const { token } = useParams<{ token: string }>();
    const [info, setInfo] = useState<any>(null);
    const [loading, setLoading] = useState(true);
    const [submitting, setSubmitting] = useState(false);
    const [error, setError] = useState('');
    const [success, setSuccess] = useState(false);
    const sigCanvas = useRef<SignatureCanvas>(null);

    useEffect(() => {
        const fetchInfo = async () => {
            try {
                // We're using standard fetch so we don't have to deal with missing auth tokens for public endpoints
                const res = await fetch(`${import.meta.env.VITE_API_URL || 'http://localhost:3000'}/api/signatures/s/${token}`);
                const data = await res.json();
                if (data.success) {
                    setInfo(data.data);
                } else {
                    setError(data.error || 'No se pudo cargar la información');
                }
            } catch (err: any) {
                setError(err.message || 'Error de conexión');
            } finally {
                setLoading(false);
            }
        };
        fetchInfo();
    }, [token]);

    const clearSignature = () => {
        sigCanvas.current?.clear();
    };

    const submitSignature = async () => {
        if (sigCanvas.current?.isEmpty()) {
            alert('Por favor, ingrese su firma.');
            return;
        }
        
        setSubmitting(true);
        try {
            // Remove the data:image/png;base64, part
            const base64Str = sigCanvas.current?.toDataURL('image/png').split(',')[1];
            
            const payload = {
                signature_base64: base64Str,
                browser: navigator.userAgent,
                operating_system: navigator.platform,
                ip: '127.0.0.1', // Should be fetched properly or backend captures it
                user_agent: navigator.userAgent
            };

            const res = await fetch(`${import.meta.env.VITE_API_URL || 'http://localhost:3000'}/api/signatures/s/${token}`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(payload)
            });
            const data = await res.json();

            if (data.success) {
                setSuccess(true);
            } else {
                setError(data.error || 'Error al enviar la firma');
            }
        } catch (err: any) {
            setError(err.message || 'Error de red al enviar la firma');
        } finally {
            setSubmitting(false);
        }
    };

    if (loading) return <div className="p-8 text-center">Cargando...</div>;
    if (error && !info) return <div className="p-8 text-center text-red-500">{error}</div>;
    if (success) return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50">
            <div className="p-8 bg-white rounded-lg shadow-md max-w-md w-full text-center">
                <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-4">
                    <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                </div>
                <h2 className="text-2xl font-bold text-gray-800 mb-2">¡Firma Completada!</h2>
                <p className="text-gray-600">Su firma ha sido registrada exitosamente. Ya puede cerrar esta pestaña.</p>
            </div>
        </div>
    );

    return (
        <div className="flex flex-col items-center min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
            <div className="max-w-3xl w-full space-y-8">
                <div className="text-center">
                    <h2 className="mt-6 text-3xl font-extrabold text-gray-900">
                        Firma Electrónica de Contrato
                    </h2>
                    <p className="mt-2 text-sm text-gray-600">
                        Por favor, lea el contrato y dibuje su firma en el recuadro inferior.
                    </p>
                </div>

                <div className="bg-white shadow overflow-hidden sm:rounded-lg p-6">
                    <div className="mb-6">
                        <h3 className="text-lg leading-6 font-medium text-gray-900">
                            Detalles del Contrato
                        </h3>
                        <p className="mt-1 max-w-2xl text-sm text-gray-500">
                            ID: {info?.contract_id}
                        </p>
                    </div>

                    <div className="border border-gray-200 rounded p-4 h-64 overflow-y-auto mb-8 bg-gray-50 text-gray-700 text-sm">
                        {info?.contract_snapshot ? (
                            <div>
                                <h4 className="font-bold text-center mb-4 text-lg">CONTRATO DE LOCACIÓN</h4>
                                <div className="mb-4">
                                    <p><strong>Inmueble:</strong> {info.contract_snapshot.property_address || 'No especificado'}</p>
                                    <p><strong>Locador:</strong> {
                                        info.contract_snapshot.participants?.find((p: any) => p.p_role === 'LANDLORD')?.client_name || 'No especificado'
                                    }</p>
                                    <p><strong>Locatario:</strong> {
                                        info.contract_snapshot.participants?.find((p: any) => p.p_role === 'TENANT')?.client_name || 'No especificado'
                                    }</p>
                                    <p><strong>Fecha de Inicio:</strong> {info.contract_snapshot.contract?.start_date || '...'}</p>
                                    <p><strong>Fecha de Finalización:</strong> {info.contract_snapshot.contract?.end_date || '...'}</p>
                                    <p><strong>Monto Inicial del Alquiler:</strong> ${info.contract_snapshot.contract?.original_rent_amount || 0}</p>
                                </div>
                                
                                {info.contract_snapshot.clauses?.length ? (
                                    info.contract_snapshot.clauses.map((clause: any, index: number) => {
                                        let body = clause.body || '';
                                        body = body.replace('[MONTO_ALQUILER]', `$${info.contract_snapshot.contract?.original_rent_amount || 0}`);
                                        body = body.replace('[FECHA_INICIO]', info.contract_snapshot.contract?.start_date || '...');
                                        body = body.replace('[FECHA_FIN]', info.contract_snapshot.contract?.end_date || '...');
                                        
                                        return (
                                            <div key={index} className="mb-4">
                                                <h5 className="font-semibold mb-1">{clause.title}</h5>
                                                <p className="whitespace-pre-wrap">{body}</p>
                                            </div>
                                        );
                                    })
                                ) : (
                                    <p className="text-center italic mt-4">Este contrato no tiene cláusulas registradas.</p>
                                )}
                            </div>
                        ) : (
                            <p className="text-center italic">Cargando contenido del contrato...</p>
                        )}
                    </div>

                    <div className="border-2 border-dashed border-gray-300 rounded-lg p-4 bg-gray-50">
                        <SignatureCanvas
                            ref={sigCanvas}
                            penColor="black"
                            canvasProps={{
                                className: 'w-full h-48 rounded bg-white border border-gray-200 shadow-sm'
                            }}
                        />
                    </div>
                    
                    <div className="mt-4 flex justify-between">
                        <button
                            onClick={clearSignature}
                            type="button"
                            className="inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none"
                        >
                            Limpiar
                        </button>
                        <button
                            onClick={submitSignature}
                            disabled={submitting}
                            type="button"
                            className="inline-flex items-center px-6 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none disabled:opacity-50"
                        >
                            {submitting ? 'Enviando...' : 'Firmar y Enviar'}
                        </button>
                    </div>
                    {error && <p className="mt-2 text-sm text-red-600">{error}</p>}
                </div>
            </div>
        </div>
    );
}
