use gdnative_project_utils::*;

pub fn _main() -> Result<(), Box<dyn std::error::Error>> {
  let classes = scan_crate(".")?;

  Generator::new().godot_project_dir("../").build(classes)?;

  Ok(())
}
