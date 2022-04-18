fn main() {
    pkg_config::Config::new()
        .atleast_version("1.2")
        .probe("z")
        .unwrap();
    let src = [
        // common files
        "esp32-camera/driver/esp_camera.c",
        "esp32-camera/driver/cam_hal.c",
        "esp32-camera/driver/sccb.c",
        "esp32-camera/driver/sensor.c",
        "esp32-camera/sensors/ov2640.c",
        "esp32-camera/sensors/ov3660.c",
        "esp32-camera/sensors/ov5640.c",
        "esp32-camera/sensors/ov7725.c",
        "esp32-camera/sensors/ov7670.c",
        "esp32-camera/sensors/nt99141.c",
        "esp32-camera/sensors/gc0308.c",
        "esp32-camera/sensors/gc2145.c",
        "esp32-camera/sensors/gc032a.c",
        "esp32-camera/sensors/bf3005.c",
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
        "esp32-camera/conversions/include",

    ];
    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .includes(inc.iter())
        .flag("-Wno-unused-parameter");
//        .define("USE_ZLIB", None);
    build.compile("esp32-camera");
}