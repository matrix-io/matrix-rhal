use esp_idf_build as build;

fn main() {
    //build::build_native().unwrap();
    build::print_link_search();
    // Write `image.sh` used by `image-project`
    build::esptool_write_script().unwrap();
}
