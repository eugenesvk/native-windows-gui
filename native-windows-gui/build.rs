extern crate build;
fn main() {
    if cfg!(feature = "file-dialog") {
        build::link("shell32", true);
    }
    embed_resource::compile("embed_resources.rc");
}
