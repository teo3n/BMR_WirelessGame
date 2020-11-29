use std::{env, fs};

fn main() {
    println!("cargo:rustc-link-search={}", "/mnt/g/projektit/ESP8266/Espressif_toolchain/ESP8266_NONOS_SDK/lib");

}
