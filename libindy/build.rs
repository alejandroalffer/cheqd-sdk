use regex::Regex;
use std::env;
use std::fs;
use std::fs::{create_dir_all, remove_dir_all};
use std::{
    ffi::OsStr,
    io,
    path::{Path, PathBuf},
    process,
};
use walkdir::WalkDir;

fn main() {
    let target = env::var("TARGET").unwrap();
    println!("target={}", target);

    let sodium_static = env::var("CARGO_FEATURE_SODIUM_STATIC").ok();
    println!("sodium_static={:?}", sodium_static);

    if sodium_static.is_some() {
        println!("cargo:rustc-link-lib=static=sodium");
    }

    build_proto();

    if target.find("-windows-").is_some() {
        // do not build c-code on windows, use binaries
        let output_dir = env::var("OUT_DIR").unwrap();
        let prebuilt_dir = env::var("INDY_PREBUILT_DEPS_DIR").unwrap();

        let dst = Path::new(&output_dir[..]).join("..\\..\\..");
        let prebuilt_lib = Path::new(&prebuilt_dir[..]).join("lib");

        println!("cargo:rustc-link-search=native={}", prebuilt_dir);
        println!("cargo:rustc-flags=-L {}\\lib", prebuilt_dir);
        println!("cargo:include={}\\include", prebuilt_dir);

        let files = vec![
            "libeay32md.dll",
            "libsodium.dll",
            "libzmq.dll",
            "ssleay32md.dll",
        ];
        for f in files.iter() {
            if let Ok(_) = fs::copy(&prebuilt_lib.join(f), &dst.join(f)) {
                println!(
                    "copy {} -> {}",
                    &prebuilt_lib.join(f).display(),
                    &dst.join(f).display()
                );
            }
        }
    } else if target.find("linux-android").is_some() {
        //statically link files
        let openssl = match env::var("OPENSSL_LIB_DIR") {
            Ok(val) => val,
            Err(..) => match env::var("OPENSSL_DIR") {
                Ok(dir) => Path::new(&dir[..])
                    .join("lib")
                    .to_string_lossy()
                    .into_owned(),
                Err(..) => {
                    panic!("Missing required environment variables OPENSSL_DIR or OPENSSL_LIB_DIR")
                }
            },
        };

        let sodium = match env::var("SODIUM_LIB_DIR") {
            Ok(val) => val,
            Err(..) => panic!("Missing required environment variable SODIUM_LIB_DIR"),
        };

        let zmq = match env::var("LIBZMQ_LIB_DIR") {
            Ok(val) => val,
            Err(..) => match env::var("LIBZMQ_PREFIX") {
                Ok(dir) => Path::new(&dir[..])
                    .join("lib")
                    .to_string_lossy()
                    .into_owned(),
                Err(..) => {
                    panic!("Missing required environment variables LIBZMQ_PREFIX or LIBZMQ_LIB_DIR")
                }
            },
        };

        println!("cargo:rustc-link-search=native={}", openssl);
        println!("cargo:rustc-link-lib=static=crypto");
        println!("cargo:rustc-link-lib=static=ssl");
        println!("cargo:rustc-link-search=native={}", sodium);
        println!("cargo:rustc-link-lib=static=sodium");
        println!("cargo:rustc-link-search=native={}", zmq);
        println!("cargo:rustc-link-lib=static=zmq");
    }
}

/// ------ PROTO ------

const COSMOS_SDK_DIR: &str = "cosmos-sdk-go";
const VERIMCOSMOS_DIR: &str = "verim-cosmos";

fn build_proto() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let proto_dir: PathBuf = format!("{}/prost", out_dir).parse().unwrap();

    if proto_dir.exists() {
        fs::remove_dir_all(proto_dir.clone()).unwrap();
    }

    fs::create_dir(proto_dir.clone()).unwrap();

    compile_protos(&proto_dir);
    // compile_proto_services(&proto_dir);
}

fn compile_protos(out_dir: &Path) {
    let sdk_dir = Path::new(COSMOS_SDK_DIR);
    let verimcosmos_dir = Path::new(VERIMCOSMOS_DIR);

    println!(
        "[info] Compiling .proto files to Rust into '{}'...",
        out_dir.display()
    );

    let root = env!("CARGO_MANIFEST_DIR");

    // Paths
    let proto_paths = [
        // format!("{}/../proto/definitions/mock", root),
        // format!("{}/proto/ibc", sdk_dir.display()),
        // format!("{}/proto/cosmos/tx", sdk_dir.display()),
        // format!("{}/proto/cosmos/bank", sdk_dir.display()),
        // format!("{}/proto/cosmos/base", sdk_dir.display()),
        // format!("{}/proto/cosmos/staking", sdk_dir.display()),
        format!("{}/proto/verimcosmos", verimcosmos_dir.display()),
    ];

    let proto_includes_paths = [
        format!("{}/proto", sdk_dir.display()),
        format!("{}/proto", verimcosmos_dir.display()),
        format!("{}/third_party/proto", sdk_dir.display()),
    ];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    for proto_path in &proto_paths {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    // Compile all proto files
    let mut config = prost_build::Config::default();
    config.out_dir(out_dir);
    config.extern_path(".tendermint", "::tendermint_proto");
    config.extern_path(".cosmos", "cosmos_sdk::proto::cosmos");

    if let Err(e) = config.compile_protos(&protos, &includes) {
        eprintln!("[error] couldn't compile protos: {}", e);
        panic!("protoc failed!");
    }
}

fn compile_proto_services(out_dir: impl AsRef<Path>) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sdk_dir = PathBuf::from(COSMOS_SDK_DIR);
    let verimcosmos_dir = PathBuf::from(VERIMCOSMOS_DIR);

    let proto_includes_paths = [
        sdk_dir.join("proto"),
        sdk_dir.join("third_party/proto"),
        verimcosmos_dir.join("proto"),
    ];

    // List available paths for dependencies
    let includes = proto_includes_paths
        .iter()
        .map(|p| p.as_os_str().to_os_string())
        .collect::<Vec<_>>();

    let proto_services_path = [
        // sdk_dir.join("proto/cosmos/auth/v1beta1/query.proto"),
        // sdk_dir.join("proto/cosmos/staking/v1beta1/query.proto"),
        // sdk_dir.join("proto/cosmos/bank/v1beta1/query.proto"),
        // sdk_dir.join("proto/cosmos/bank/v1beta1/tx.proto"),
        // sdk_dir.join("proto/cosmos/tx/v1beta1/service.proto"),
        // sdk_dir.join("proto/cosmos/tx/v1beta1/tx.proto"),
        verimcosmos_dir.join("proto/verimcosmos/tx.proto"),
        verimcosmos_dir.join("proto/verimcosmos/query.proto"),
    ];

    // List available paths for dependencies
    let services = proto_services_path
        .iter()
        .map(|p| p.as_os_str().to_os_string())
        .collect::<Vec<_>>();

    // Compile all proto client for GRPC services
    println!("[info] Compiling proto clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .format(true)
        .out_dir(out_dir)
        .compile(&services, &includes)
        .unwrap();

    println!("[info ] => Done!");
}
