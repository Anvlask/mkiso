use serde::Deserialize;
use toml::Table;

#[derive(Deserialize, Debug)]
pub struct PartitionTemplate {
  pub name: String,
  pub parttype: String,
  pub size: i64,
  pub root: String,
}

impl PartitionTemplate {
  pub fn from_table(name: &str, table: &Table) -> Self {
    Self {
      name: name.into(),
      parttype: table
        .get("type")
        .expect(&format!("Unspecified partition type for {name}"))
        .as_str()
        .expect(&format!("Invalid partition type for {name}"))
        .into(),
      size: table
        .get("size")
        .expect(&format!("Unspecified size for {name}"))
        .as_integer()
        .expect(&format!("Invalid size for {name}")),
      root: table
        .get("root")
        .expect(&format!("Unspecified root for {name}"))
        .as_str()
        .expect(&format!("Invalid root for {name}"))
        .into(),
    }
  }
}
