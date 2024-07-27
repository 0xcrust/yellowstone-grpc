fn main() -> anyhow::Result<()> {
    if std::env::var("PROTOC").is_err() {
        std::env::set_var("PROTOC", protobuf_src::protoc());
    }
    tonic_build::compile_protos("proto/yellowstone-log.proto")?;
    Ok(())
}
