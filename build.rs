fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::compile_protos("api/account/authorization_service.proto")?;
  Ok(())
}