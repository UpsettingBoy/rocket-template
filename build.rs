use npm_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_type = match &std::env::var("PROFILE").unwrap()[..] {
        "release" => "production",
        _ => "development",
    };

    let npm_path = std::env::current_dir().unwrap().join("public");

    let npm_status = NpmEnv::default()
        .with_env("NODE_ENV", build_type)
        .set_path(npm_path)
        .init()
        .install(None)
        .run("css")
        .exec()?;

    if !npm_status.success() {
        println!("cargo:warning=npm failed with: {}", npm_status);
    }

    Ok(())
}
