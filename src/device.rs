use super::Deserialize;

pub type DeviceFamily = String;
pub type DeviceBrand = String;
pub type DeviceModel = String;

#[derive(Debug, Deserialize, Clone)]
pub struct Device {
  family: DeviceFamily,
  brand: Option<DeviceBrand>,
  model: Option<DeviceModel>,
}

impl Default for Device {
  fn default() -> Device {
    Device {
      family: "Other".to_string(),
      brand: None,
      model: None,
    }
  }
}