// fn main() {
//     gear_wasm_builder::build();
// }

use game_io::PMetadata;
fn main() {
    gear_wasm_builder::build_with_metadata::<PMetadata>();
}