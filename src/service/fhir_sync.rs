use crate::proto::fhir_sync::{
    fhir_sync_server::{FhirSync, FhirSyncServer}, // FhirSync is the service trait, FhirSyncServer<T> is the gRPC implementation wrapper
    PatientRef,
    Patient,
    PatientAck,
    ChangeSet,
    Ack,
};

use tonic::{Request, Response, Status};
use tokio_stream::wrappers::ReceiverStream;


#[derive(Debug, Default)]
pub struct MyFhirSyncService;

#[tonic::async_trait]
impl FhirSync for MyFhirSyncService {
    async fn get_patient(
        &self,
        request: tonic::Request<PatientRef>,
    ) -> Result<tonic::Response<Patient>, tonic::Status> {
        let patient_id = request.into_inner().id;
        let patient = Patient {
            id: patient_id,
            ..Default::default()
        };
        Ok(tonic::Response::new(patient))
    }

    async fn upsert_patient(
        &self,
        request: tonic::Request<Patient>,
    ) -> Result<tonic::Response<PatientAck>, tonic::Status> {
        let id = request.into_inner().id;
        Ok(tonic::Response::new(PatientAck {
            id,
            status: "ok".into(),
        }))
    }

    type StreamChangesStream =
        tokio_stream::wrappers::ReceiverStream<Result<Ack, tonic::Status>>;

    async fn stream_changes(
        &self,
        request: tonic::Request<tonic::Streaming<ChangeSet>>,
    ) -> Result<tonic::Response<Self::StreamChangesStream>, tonic::Status> {
        let mut stream = request.into_inner();
        let (tx, rx) = tokio::sync::mpsc::channel(32);

        tokio::spawn(async move {
            while let Ok(Some(_change)) = stream.message().await {
                let _ = tx.send(Ok(Ack {
                    message: "got it".into(),
                }))
                .await;
            }
        });

        Ok(tonic::Response::new(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        ))
    }
}
