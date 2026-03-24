use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let proto_root = manifest_dir.join("../../../proto/vibetown");
    let proto_root = proto_root.canonicalize()?;

    let protos = &[
        proto_root.join("orchestration/v1/orchestration.proto"),
        proto_root.join("feed/v1/feed.proto"),
        proto_root.join("mail/v1/mail.proto"),
    ];

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile_protos(protos, &[&proto_root])?;

    Ok(())
}
