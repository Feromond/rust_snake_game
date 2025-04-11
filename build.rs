use {
    std::{
        env,
        io,
        fs,
        path::PathBuf,
    },
    fs_extra,
    winresource::WindowsResource,
};

fn main() -> io::Result<()> {
    // --- Resource Copying Logic --- Start ---
    println!("cargo:rerun-if-changed=resources"); // Re-run build script if resources change

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // Determine the correct target directory (usually 3 levels up from OUT_DIR)
    let target_dir = out_dir
        .ancestors()
        .nth(3)
        .unwrap_or_else(|| panic!("Failed to determine target directory from OUT_DIR"))
        .to_path_buf();

    let resources_src = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("resources");
    let resources_dest = target_dir.join("resources");

    println!(
        "Build: Attempting to copy resources from: {:?} to: {:?}",
        resources_src,
        resources_dest
    );

    // Ensure the source directory exists
    if !resources_src.exists() {
        panic!("Build: Source resources directory does not exist: {:?}", resources_src);
    }
     if !resources_src.is_dir() {
        panic!("Build: Source resources path is not a directory: {:?}", resources_src);
    }

    // Remove the destination directory if it exists to ensure a clean copy
    if resources_dest.exists() {
        if resources_dest.is_dir() {
             fs::remove_dir_all(&resources_dest)
                .expect("Build: Failed to remove existing resources directory");
        } else {
             fs::remove_file(&resources_dest)
                .expect("Build: Failed to remove existing resources file (conflicting name)");
        }
        println!("Build: Removed existing destination: {:?}", resources_dest);
    }

    // Create the destination directory before copying contents into it
    fs::create_dir_all(&resources_dest).expect("Build: Failed to create destination resources directory");
    println!("Build: Ensured destination directory exists: {:?}", resources_dest);

    // Copy the contents of the resources directory
    let mut options = fs_extra::dir::CopyOptions::new();
    options.copy_inside = true; // Copy the *contents* of resources_src into resources_dest
    options.content_only = true; // Double ensure only contents are copied

    fs_extra::dir::copy(&resources_src, &resources_dest, &options)
         .map_err(|e| {
            eprintln!("Build Error: Error copying resources: {}", e);
            io::Error::new(io::ErrorKind::Other, format!("Failed to copy resources: {}", e))
        })?;

    println!(
        "Build: Successfully copied contents from {:?} to {:?}",
        resources_src,
        resources_dest
    );
    // --- Resource Copying Logic --- End ---


    // --- Windows Icon Logic --- Start ---
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        println!("Build: Attempting to set Windows icon...");
        let icon_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("snake.ico");
        if icon_path.exists() {
             WindowsResource::new()
                .set_icon(icon_path.to_str().unwrap())
                .compile()?;
             println!("Build: Windows icon set successfully from {:?}.", icon_path);
        } else {
            println!("Build Warning: snake.ico not found at {:?}, skipping icon setting.", icon_path);
        }
    }
    // --- Windows Icon Logic --- End ---

    Ok(())
}
