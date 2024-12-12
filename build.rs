use anyhow::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        .bytes(&["."])
        .out_dir("src/pb")
        .compile_protos(&["chat.proto"], &["."])?;

    Ok(())
}
