use crate::core::contracts::genpdf_impl::GenPdfGenerator;
use crate::core::contracts::pdf_generator::PdfGenerator;
use async_trait::async_trait;
use genpdf::{elements, Alignment, Document, Element as _, SimplePageDecorator};
use serde_json::Value;
use std::io::Cursor;

pub struct SignedPdfGenerator {
    base_generator: GenPdfGenerator,
    font_dir: String,
}

impl SignedPdfGenerator {
    pub fn new(font_dir: &str) -> Result<Self, String> {
        let base_generator = GenPdfGenerator::new(font_dir)?;
        Ok(Self { base_generator, font_dir: font_dir.to_string() })
    }

    pub async fn generate_signed_contract(
        &self,
        contract_snapshot: Value,
        signatures: Vec<Value>, // Contains names, dates, IPs, hashes, verification codes
    ) -> Result<Vec<u8>, String> {
        // First, we generate the base contract document
        // We will build a new document with the base generator's fonts, but unfortunately we cannot easily just "append" to a rendered Vec<u8> using genpdf.
        // We must re-render the whole document with the signatures attached at the end.
        
        let mut doc = Document::new(
            genpdf::fonts::from_files(&self.font_dir, "Roboto", None)
                .map_err(|e| format!("Failed to load fonts: {}", e))?
        );
        doc.set_title("Contrato de Locación Firmado");

        let mut decorator = SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_page_decorator(decorator);

        // Header Title
        let mut title = elements::Paragraph::new("CONTRATO DE LOCACIÓN");
        title.set_alignment(Alignment::Center);
        doc.push(title);
        doc.push(elements::Break::new(2));

        let c = contract_snapshot.get("contract").and_then(|v| v.as_object());
        let start_date = c.and_then(|c| c.get("start_date")).and_then(|v| v.as_str()).unwrap_or("...");
        let end_date = c.and_then(|c| c.get("end_date")).and_then(|v| v.as_str()).unwrap_or("...");
        let rent_amount = c.and_then(|c| c.get("original_rent_amount")).and_then(|v| v.as_f64()).unwrap_or(0.0);
        let property_address = contract_snapshot.get("property_address").and_then(|v| v.as_str()).unwrap_or("...");

        // Render Preamble
        let mut landlord = "No especificado".to_string();
        let mut tenant = "No especificado".to_string();

        if let Some(participants) = contract_snapshot.get("participants").and_then(|v| v.as_array()) {
            for p in participants {
                if let Some(p_obj) = p.as_object() {
                    let role = p_obj.get("p_role").and_then(|v| v.as_str()).unwrap_or("");
                    let name = p_obj.get("client_name").and_then(|v| v.as_str()).unwrap_or("");
                    if role == "LANDLORD" { landlord = name.to_string(); }
                    if role == "TENANT" { tenant = name.to_string(); }
                }
            }
        }
        
        let preamble_text = format!(
            "Entre el Sr./Sra. {}, en adelante denominado EL LOCADOR, y el Sr./Sra. {}, en adelante denominado EL LOCATARIO, convienen en celebrar el presente Contrato de Locación respecto del inmueble sito en {}. El mismo tendrá una vigencia desde el {} hasta el {}, pactándose un monto inicial de alquiler de ${:.2}, sujeto a las siguientes cláusulas y condiciones:",
            landlord, tenant, property_address, start_date, end_date, rent_amount
        );
        let mut preamble = elements::Paragraph::new(preamble_text);
        preamble.set_alignment(Alignment::Left);
        doc.push(preamble);
        
        doc.push(elements::Break::new(2));
        
        // Render Clauses
        if let Some(clauses) = contract_snapshot.get("clauses").and_then(|v| v.as_array()) {
            for clause in clauses {
                if let Some(clause_obj) = clause.as_object() {
                    let clause_title = clause_obj.get("title").and_then(|v| v.as_str()).unwrap_or("");
                    let mut clause_body = clause_obj.get("body").and_then(|v| v.as_str()).unwrap_or("").to_string();

                    clause_body = clause_body.replace("[MONTO_ALQUILER]", &format!("{:.2}", rent_amount));
                    clause_body = clause_body.replace("[FECHA_INICIO]", start_date);
                    clause_body = clause_body.replace("[FECHA_FIN]", end_date);

                    let mut p_title = elements::Paragraph::new(clause_title);
                    p_title.set_alignment(Alignment::Left);
                    doc.push(p_title);

                    let mut p_body = elements::Paragraph::new(clause_body);
                    p_body.set_alignment(Alignment::Left);
                    doc.push(p_body);
                    doc.push(elements::Break::new(1));
                }
            }
        }

        // Add Signatures Block
        doc.push(elements::PageBreak::new());
        let mut sig_title = elements::Paragraph::new("ANEXO DE FIRMAS ELECTRÓNICAS");
        sig_title.set_alignment(Alignment::Center);
        doc.push(sig_title);
        doc.push(elements::Break::new(2));

        for sig in signatures {
            let name = sig.get("name").and_then(|v| v.as_str()).unwrap_or("Desconocido");
            let date = sig.get("signed_at").and_then(|v| v.as_str()).unwrap_or("");
            let ip = sig.get("ip").and_then(|v| v.as_str()).unwrap_or("");
            let browser = sig.get("browser").and_then(|v| v.as_str()).unwrap_or("");
            let hash = sig.get("signature_sha256").and_then(|v| v.as_str()).unwrap_or("");
            let verification_code = sig.get("verification_code").and_then(|v| v.as_str()).unwrap_or("");
            
            doc.push(elements::Paragraph::new(format!("Firmante: {}", name)));
            doc.push(elements::Paragraph::new(format!("Fecha: {}", date)));
            doc.push(elements::Paragraph::new(format!("IP: {}", ip)));
            doc.push(elements::Paragraph::new(format!("Navegador: {}", browser)));
            doc.push(elements::Paragraph::new(format!("Código de Verificación: {}", verification_code)));
            doc.push(elements::Paragraph::new(format!("Hash SHA256: {}", hash)));
            
            let verify_url = format!("https://inmonea.agentech.ar/api/v2/signatures/verify/{}", verification_code);
            doc.push(elements::Paragraph::new(format!("QR Verification Link: {}", verify_url)));
            
            if let Ok(code) = qrcode::QrCode::new(verify_url.as_bytes()) {
                let image_buffer = code.render::<image::Luma<u8>>().build();
                let dynamic_image = image::DynamicImage::ImageLuma8(image_buffer);
                if let Ok(mut genpdf_image) = elements::Image::from_dynamic_image(dynamic_image) {
                    genpdf_image.set_alignment(Alignment::Center);
                    doc.push(genpdf_image);
                }
            }
            doc.push(elements::Break::new(1));
        }

        let mut buf = Cursor::new(Vec::new());
        doc.render(&mut buf)
            .map_err(|e| format!("Failed to render Signed Contract PDF: {}", e))?;

        Ok(buf.into_inner())
    }
}
