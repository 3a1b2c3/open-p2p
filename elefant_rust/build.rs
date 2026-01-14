use std::env;

fn main() {
    let ffmpeg_dir = "C:\\bin\\ffmpeg-7.1-full_build-shared";
    
    println!("cargo:rustc-env=FFMPEG_DIR={}", ffmpeg_dir);

    // Get the FFMPEG_DIR from environment or use default
    let ffmpeg_dir = env::var("FFMPEG_DIR").unwrap_or_else(|_| {
        println!("cargo:warning=FFMPEG_DIR environment variable not set. Using default path.");
        ffmpeg_dir.to_string()
    });

    // let ffmpeg_path = PathBuf::from(&ffmpeg_dir);

    // Set include paths for FFmpeg
    println!("cargo:rustc-link-search=native={}/lib", ffmpeg_dir);
    println!("cargo:rustc-link-search=native={}/include", ffmpeg_dir);
    
    // Set LIBCLANG_PATH to help bindgen find FFmpeg headers
    println!("cargo:rustc-env=LIBCLANG_PATH={}", ffmpeg_dir);
    
    // Add include directory for C compiler via environment
    println!("cargo:rustc-env=INCLUDE={}\\include", ffmpeg_dir);
    
    // Pass include path to clang
    println!("cargo:rustc-env=CFLAGS=-I{}\\include", ffmpeg_dir);
    println!("cargo:rustc-env=CXXFLAGS=-I{}\\include", ffmpeg_dir);
    
    // Link against FFmpeg libraries
    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=swscale");

    // Tell Cargo to rerun if these change
    println!("cargo:rerun-if-changed={}/include", ffmpeg_dir);
    println!("cargo:rerun-if-changed={}/lib", ffmpeg_dir);
}