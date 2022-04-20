use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system nvpair of zfs
    // shared library.
    println!("cargo:rustc-link-lib=esp32-camera");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(r#".\esp32-camera\driver\include\esp_camera.h"#)
        .allowlist_type(r#"(\w*camera_\w*)"#)
        .allowlist_function(r#"(\w*esp_camera\w*)"#)
        .blocklist_file(r#".\.embuild\*"#)
        .clang_args(vec![r#"-I.\.embuild\platformio\packages\framework-espidf\components\esp_common\include\"#,
            r#"-I.\.embuild\platformio\packages\toolchain-xtensa-esp32\xtensa-esp32-elf\include\"#,
            r#"-I.\.embuild\platformio\packages\framework-espidf\components\driver\include\"#,
            r#"-I.\.embuild\platformio\packages\framework-espidf\components\esp_system\include\"#,
            r#"-I.\.embuild\platformio\packages\framework-espidf\components\soc\esp32\include\"#,
            r#"-I.\.embuild\platformio\packages\framework-espidf\components\hal\include\"#,
            r#"-I.\.embuild\platformio\packages\framework-espidf\components\xtensa\include\"#,
            "-I.",
            r#"-I.\.embuild\platformio\packages\framework-espidf\components\soc\include\"#,
            "-DCONFIG_IDF_TARGET_ESP32",
            r#"-I.\.embuild\platformio\packages\framework-espidf\components\esp32\include\"#,
            r#"-I.\.embuild\platformio\packages\framework-espidf\components\esp_rom\include\"#,
            r#"-I.\esp32-camera\conversions\include\"#,
            r#"-I.\esp32-camera\driver\include\"#])
        // Disable rustfmt, as it is not available on the esp toolchain
        .rustfmt_bindings(false)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("./src");
//    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");


//    pkg_config::Config::new()
//        .atleast_version("1.2")
//        .probe("esp-idf")
//        .unwrap();
    let src = [
        // common files
        "esp32-camera/driver/esp_camera.c",
        "esp32-camera/driver/cam_hal.c",
        "esp32-camera/driver/sccb.c",
        "esp32-camera/driver/sensor.c",
        "esp32-camera/sensors/ov2640.c",
//        "esp32-camera/sensors/ov3660.c",
//        "esp32-camera/sensors/ov5640.c",
//        "esp32-camera/sensors/ov7725.c",
//        "esp32-camera/sensors/ov7670.c",
//        "esp32-camera/sensors/nt99141.c",
//        "esp32-camera/sensors/gc0308.c",
//        "esp32-camera/sensors/gc2145.c",
//        "esp32-camera/sensors/gc032a.c",
//        "esp32-camera/sensors/bf3005.c",
        "esp32-camera/conversions/yuv.c",
        "esp32-camera/conversions/to_jpg.cpp",
        "esp32-camera/conversions/to_bmp.c",
        "esp32-camera/conversions/jpge.cpp",
        "esp32-camera/conversions/esp_jpg_decode.c",

        // esp32
        "esp32-camera/target/xclk.c",
        "esp32-camera/target/esp32/ll_cam.c",
    ];
    let inc = [
        "esp32-camera/driver/include",
        "esp32-camera/driver/private_include",
        "esp32-camera/conversions/include",
//        ".embuild/platformio/packages/framework-espidf/components/freertos/include",
//        ".embuild/platformio/packages/framework-espidf/components/heap/include",
    ];
    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .includes(inc.iter())
        .target("xtensa-esp32-elf")
        .shared_flag(true)
        .compiler(PathBuf::from("./.embuild/platformio/packages/toolchain-xtensa-esp32/bin/xtensa-esp32-elf-cc.exe"))
        .flag("-Wno-unused-parameter");
//        .define("USE_ZLIB", None);
    build.compile("esp32-camera");
}