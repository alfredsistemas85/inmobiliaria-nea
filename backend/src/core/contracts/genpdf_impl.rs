use async_trait::async_trait;
use genpdf::{elements, fonts, Alignment, Document, Element as _, SimplePageDecorator};
use std::io::Cursor;
use crate::core::contracts::pdf_generator::{CertificateData, PdfGenerator};

pub struct GenPdfGenerator {
    font_family: fonts::FontFamily<fonts::FontData>,
}

impl GenPdfGenerator {
    pub fn new(font_dir: &str) -> Result<Self, String> {
        let font_family = fonts::from_files(font_dir, "Roboto", None)
            .map_err(|e| format!("Failed to load fonts: {}", e))?;
        
        Ok(Self { font_family })
    }
}

#[async_trait]
impl PdfGenerator for GenPdfGenerator {
    async fn generate_adjustment_certificate(&self, data: CertificateData) -> Result<Vec<u8>, String> {
        let mut doc = Document::new(self.font_family.clone());
        doc.set_title("Certificado de Actualización de Alquiler");

        let mut decorator = SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_page_decorator(decorator);

        // Header
        let mut header = elements::Paragraph::new(data.real_estate_name);
        header.set_alignment(Alignment::Center);
        doc.push(header);
        
        doc.push(elements::Break::new(1));
        
        let mut title = elements::Paragraph::new("Certificado de Actualización de Alquiler");
        title.set_alignment(Alignment::Center);
        doc.push(title);

        doc.push(elements::Break::new(2));

        // Details
        doc.push(elements::Paragraph::new(format!("Inmueble: {}", data.property_address)));
        doc.push(elements::Paragraph::new(format!("Propietario: {}", data.owner_name)));
        doc.push(elements::Paragraph::new(format!("Inquilino: {}", data.tenant_name)));
        doc.push(elements::Paragraph::new(format!("Contrato ID: {}", data.contract_id)));
        
        doc.push(elements::Break::new(1));

        doc.push(elements::Paragraph::new(format!("Monto Anterior: ${}", data.previous_amount)));
        doc.push(elements::Paragraph::new(format!("Nuevo Monto: ${}", data.new_amount)));
        doc.push(elements::Paragraph::new(format!("Método Aplicado: {}", data.method)));
        doc.push(elements::Paragraph::new(format!("Porcentaje de Ajuste: {}%", data.percentage)));
        doc.push(elements::Paragraph::new(format!("Fecha Efectiva: {}", data.effective_date)));

        doc.push(elements::Break::new(2));

        doc.push(elements::Paragraph::new(format!("Aprobado por: {}", data.approver_name)));
        doc.push(elements::Paragraph::new(format!("Fecha de Emisión: {}", data.issue_date)));
        doc.push(elements::Paragraph::new(format!("Código de Auditoría: {}", data.rent_adjustment_id)));

        // Render to buffer
        let mut buf = Cursor::new(Vec::new());
        doc.render(&mut buf).map_err(|e| format!("Failed to render PDF: {}", e))?;

        Ok(buf.into_inner())
    }
}
