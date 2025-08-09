pub mod fhir_sync {
    include!("proto/arsmedicatech.fhir_sync.v1.rs");
}

pub mod google {
    pub mod fhir {
        pub mod proto {
            include!("proto/google.fhir.proto.rs");

            pub mod r5 {
                pub mod core {
                    include!("proto/google.fhir.r5.core.rs");
                }
            }
        }
    }
}