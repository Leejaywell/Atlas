use atlas_core::AtlasCore;

uniffi::include_scaffolding!("atlas");

pub fn get_core_status() -> String {
    let core = AtlasCore::new();
    core.get_status()
}
