use adapters::entities::patient::Patient as ProtoPatient;
use domain::DomainPatient;

fn example() {
    let domain_obj = DomainPatient {
        demographic_no: "DEM-123".into(),
        first_name: Some("Jane".into()),
        last_name:  Some("Doe".into()),
        date_of_birth: Some("1990-02-20".into()),
        location: Some(("Vancouver".into(), "BC".into(), "CA".into(), "V5K0A1".into())),
        sex: Some("female".into()),
        phone: Some("+1-604-123-4567".into()),
        email: Some("jane.doe@example.com".into()),
    };

    let proto_msg: ProtoPatient = domain_obj.into();

    // ready to send over gRPC
    let bytes = proto_msg.encode_to_vec();
}
