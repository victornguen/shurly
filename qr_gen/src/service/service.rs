use tonic::{Request, Response, Status};
use pb::qr_gen_pb::qr_generator_server::QrGenerator;
use crate::pb;
use crate::pb::qr_gen_pb::{QrCodeRequest, QrCodeResponse};

struct QrGen {}

#[tonic::async_trait]
impl QrGenerator for QrGen {
    async fn generate_qr_code(&self, request: Request<QrCodeRequest>) -> Result<Response<QrCodeResponse>, Status> {
        // let message = request.get_ref();
        Ok(Response::new(QrCodeResponse {
            qr_code: vec![1,2,3]
        }))
    }
}