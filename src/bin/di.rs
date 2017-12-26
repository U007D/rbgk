use quickstart_template::{self, arch};

// TODO: Auto-generate `Container` using `di::registration` module and `code_gen`.
pub struct Container {
    arch_info: arch::Arch,
}

impl Container {
    pub fn build() -> Self { Self { arch_info: quickstart_template::arch::Arch::new(), } }
    pub fn resolve_ref_arch_info(&self) -> &arch::Arch { &self.arch_info }
}
