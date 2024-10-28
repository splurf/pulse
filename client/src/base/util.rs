use crate::*;

use std::{collections::HashMap, ops::Deref};

use cpal::{
    traits::{DeviceTrait, HostTrait},
    BufferSize, StreamConfig,
};

pub type Devices<'a> = HashMap<&'a str, (Vec<Device>, Vec<Device>)>;

pub struct Device {
    inner: cpal::Device,
    name: String,
    config: StreamConfig,
}

impl Device {
    pub fn new(inner: cpal::Device, name: String, mut config: StreamConfig) -> Self {
        config.buffer_size = BufferSize::Default;

        Self {
            inner,
            name,
            config,
        }
    }

    pub fn _name(&self) -> &str {
        self.name.as_str()
    }

    pub fn config(&self) -> &StreamConfig {
        &self.config
    }

    pub fn get_host_devices<'a>() -> Result<Devices<'a>, CpalError> {
        let available_hosts = cpal::available_hosts();

        if available_hosts.is_empty() {
            return Err(cpal::HostUnavailable.into());
        }

        let mut hosts =
            HashMap::<&str, (Vec<Device>, Vec<Device>)>::with_capacity(available_hosts.len());

        let (mut input_available, mut output_available) = Default::default();

        for host_id in available_hosts {
            let host_name = host_id.name();
            hosts.insert(host_name, Default::default());

            let host = cpal::host_from_id(host_id)?;

            for inner in host.devices()? {
                let name = inner.name()?;
                let is_input = inner.supported_input_configs()?.count() > 0;
                let is_output = inner.supported_output_configs()?.count() > 0;

                let config = if is_input {
                    inner.default_input_config()?
                } else if is_output {
                    inner.default_output_config()?
                } else {
                    break;
                }
                .config();

                let device = Device::new(inner, name, config);

                hosts.entry(host_name).and_modify(|devices| {
                    if is_input {
                        input_available = true;
                        devices.0.push(device);
                    } else {
                        output_available = true;
                        devices.1.push(device);
                    }
                });
            }
        }

        if !input_available {
            return Err(CpalError::InputUnavailable);
        }

        if !output_available {
            return Err(CpalError::OutputUnavailable);
        }

        Ok(hosts)
    }
}

impl Deref for Device {
    type Target = cpal::Device;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.name))
    }
}
