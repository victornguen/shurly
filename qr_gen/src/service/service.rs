use image::{ImageFormat, Rgba};
use tonic::{Request, Response, Status};

use pb::qr_gen_pb::qr_generator_server::QrGenerator;

use crate::pb;
use crate::pb::qr_gen_pb::{Format, QrCodeRequest, QrCodeResponse};
use crate::pb::qr_gen_pb::qr_code_request::Logo;
use crate::qr::qr_gen::QrCodeBuilder;

#[derive(Debug, Default)]
pub struct QrGen {}

#[tonic::async_trait]
impl QrGenerator for QrGen {
    async fn generate_qr_code(&self, request: Request<QrCodeRequest>) -> Result<Response<QrCodeResponse>, Status> {
        let message = request.get_ref();
        let format = match &message.format() {
            Format::Png => Ok(ImageFormat::Png),
            Format::Jpeg => Ok(ImageFormat::Jpeg),
            Format::Svg => Err(Status::invalid_argument("SVG is not supported")),
        };
        if let Err(e) = format {
            return Err(e);
        }
        let logo = message.logo.clone().map_or(Ok(Vec::new()), |l|
            match l {
                Logo::Image(bytes) => Ok(bytes),
                Logo::ImageUrl(_) => Err(Status::invalid_argument("Image links are not supported")),
                Logo::PredefinedLogo(_) => Err(Status::invalid_argument("Predefined logos are not supported")),
            },
        );
        if let Err(e) = logo {
            return Err(e);
        }
        let qr_code = QrCodeBuilder::new(&message.text, &logo.unwrap())
            .with_size(message.size as u32)
            .with_bg_color(
                message.bg_color.clone().map_or(Rgba::from([255u8, 255u8, 255u8, 255u8]), |c| Rgba::from(c))
            )
            .with_logo_bg_color(
                message.logo_bg_color.clone()
                    .map_or(Rgba::from([255u8, 255u8, 255u8, 255u8]), |c| Rgba::from(c))
            )
            .with_format(format.unwrap())
            .build()
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        Ok(Response::new(QrCodeResponse { qr_code }))
    }
}

impl From<pb::qr_gen_pb::Rgba> for Rgba<u8> {
    fn from(rgba: pb::qr_gen_pb::Rgba) -> Self {
        Rgba([rgba.r as u8, rgba.g as u8, rgba.b as u8, rgba.a as u8])
    }
}