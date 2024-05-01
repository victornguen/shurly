fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/pb")
        .build_client(false)
        .compile(&["proto/qrgen.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    // tonic_build::compile_protos("pb/qrgen.pb")
    //     .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    Ok(())
}