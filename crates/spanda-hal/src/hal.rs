//! hal support for Spanda.
//!
use spanda_ast::nodes::HalMemberDecl;
pub use spanda_runtime::hal_config::HalMemberConfig;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HalBusKind {
    I2c,
    Spi,
    Uart,
    Usb,
    Ethernet,
}

pub trait HalBackend {
    fn configure(&mut self, members: &[HalMemberConfig]);
    fn read_gpio(&self, name: &str) -> bool;
    fn write_gpio(&mut self, name: &str, value: bool);
    fn read_i2c(&self, name: &str, register: u8, length: usize) -> Vec<u8>;
    fn write_i2c(&mut self, name: &str, register: u8, data: &[u8]);
    fn transfer_spi(&self, name: &str, data: &[u8]) -> Vec<u8>;
    fn read_uart(&self, name: &str) -> String;
    fn read_adc(&self, name: &str) -> f64;
    fn set_pwm(&mut self, name: &str, duty_cycle: f64);
    fn get_member(&self, name: &str) -> Option<HalMemberConfig>;
    fn list_members(&self) -> Vec<HalMemberConfig>;
}

pub struct SimHalBackend {
    members: HashMap<String, HalMemberConfig>,
    gpio_state: HashMap<String, bool>,
    i2c_registers: HashMap<String, HashMap<u8, u8>>,
    adc_values: HashMap<String, f64>,
    pwm_duty: HashMap<String, f64>,
    uart_buffers: HashMap<String, String>,
}

impl SimHalBackend {
    pub fn new() -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_hal::hal::new();

        // Assemble the struct fields and return it.
        Self {
            members: HashMap::new(),
            gpio_state: HashMap::new(),
            i2c_registers: HashMap::new(),
            adc_values: HashMap::new(),
            pwm_duty: HashMap::new(),
            uart_buffers: HashMap::new(),
        }
    }

    pub fn simulate_uart_data(&mut self, name: &str, data: &str) {
        // Description:
        //     Simulate uart data.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     data: &str
        //         Caller-supplied data.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_hal::hal::simulate_uart_data(&mut self, name, data);

        // Append into self.
        self.uart_buffers.insert(name.to_string(), data.to_string());
    }

    pub fn set_adc_value(&mut self, name: &str, value: f64) {
        // Description:
        //     Set adc value.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     value: f64
        //         Caller-supplied value.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_hal::hal::set_adc_value(&mut self, name, value);

        // Append into self.
        self.adc_values.insert(name.to_string(), value);
    }

    pub fn seed_imu_registers(&mut self, bus_name: &str, yaw: f64) {
        // Description:
        //     Seed imu registers.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     bus_name: &str
        //         Caller-supplied bus name.
        //     yaw: f64
        //         Caller-supplied yaw.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_hal::hal::seed_imu_registers(&mut self, bus_name, yaw);

        // Compute yaw int for the following logic.
        let yaw_int = yaw.floor() as i32 * 100;
        self.write_i2c(
            bus_name,
            0x1a,
            &[(yaw_int & 0xff) as u8, ((yaw_int >> 8) & 0xff) as u8],
        );
    }
}

impl Default for SimHalBackend {
    fn default() -> Self {
        // Description:
        //     Provide the default value for this type.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `default`.
        //
        // Example:
        //     let result = spanda_hal::hal::default();

        // Build the result via new.
        Self::new()
    }
}

impl HalBackend for SimHalBackend {
    fn configure(&mut self, members: &[HalMemberConfig]) {
        // Description:
        //     Configure.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     embers: &[HalMemberConfig]
        //         Caller-supplied embers.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_hal::hal::configure(&mut self, embers);

        // Call clear on the current instance.
        self.members.clear();
        self.gpio_state.clear();
        self.i2c_registers.clear();
        self.adc_values.clear();
        self.pwm_duty.clear();
        self.uart_buffers.clear();

        // Process each member.
        for m in members {
            let name = match m {
                HalMemberConfig::I2c { name, .. }
                | HalMemberConfig::Spi { name, .. }
                | HalMemberConfig::Gpio { name, .. }
                | HalMemberConfig::Pwm { name, .. }
                | HalMemberConfig::Uart { name, .. }
                | HalMemberConfig::Adc { name, .. } => name.clone(),
            };
            self.members.insert(name.clone(), m.clone());

            // Match on m and handle each case.
            match m {
                HalMemberConfig::Gpio { .. } => {
                    self.gpio_state.insert(name, false);
                }
                HalMemberConfig::Adc { .. } => {
                    self.adc_values.insert(name, 0.0);
                }
                HalMemberConfig::Pwm { .. } => {
                    self.pwm_duty.insert(name, 0.0);
                }
                HalMemberConfig::Uart { .. } => {
                    self.uart_buffers.insert(name, String::new());
                }
                HalMemberConfig::I2c { .. } => {
                    self.i2c_registers.insert(name, HashMap::new());
                }
                _ => {}
            }
        }
    }

    fn read_gpio(&self, name: &str) -> bool {
        // Description:
        //     Read gpio.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: bool
        //         Return value from `read_gpio`.
        //
        // Example:
        //     let result = spanda_hal::hal::read_gpio(&self, name);

        // Call get on the current instance.
        self.gpio_state.get(name).copied().unwrap_or(false)
    }

    fn write_gpio(&mut self, name: &str, value: bool) {
        // Description:
        //     Write gpio.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     value: bool
        //         Caller-supplied value.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_hal::hal::write_gpio(&mut self, name, value);

        // Append into self.
        self.gpio_state.insert(name.to_string(), value);
    }

    fn read_i2c(&self, name: &str, register: u8, length: usize) -> Vec<u8> {
        // Description:
        //     Read i2c.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //     register: u8
        //         Caller-supplied register.
        //     length: usize
        //         Caller-supplied length.
        //
        // Outputs:
        //     result: Vec<u8>
        //         Return value from `read_i2c`.
        //
        // Example:
        //     let result = spanda_hal::hal::read_i2c(&self, name, register, length);

        // Compute regs for the following logic.
        let regs = self.i2c_registers.get(name);
        let mut result = Vec::new();

        // Iterate over length.
        for i in 0..length {
            let val = regs
                .and_then(|r| r.get(&(register + i as u8)))
                .copied()
                .unwrap_or(0);
            result.push(val);
        }
        result
    }

    fn write_i2c(&mut self, name: &str, register: u8, data: &[u8]) {
        // Description:
        //     Write i2c.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     register: u8
        //         Caller-supplied register.
        //     data: &[u8]
        //         Caller-supplied data.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_hal::hal::write_i2c(&mut self, name, register, data);

        // Compute regs for the following logic.
        let regs = self.i2c_registers.entry(name.to_string()).or_default();

        // Iterate over enumerate with destructured elements.
        for (i, &byte) in data.iter().enumerate() {
            regs.insert(register + i as u8, byte);
        }
    }

    fn transfer_spi(&self, _name: &str, data: &[u8]) -> Vec<u8> {
        // Description:
        //     Transfer spi.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     _name: &str
        //         Caller-supplied name.
        //     data: &[u8]
        //         Caller-supplied data.
        //
        // Outputs:
        //     result: Vec<u8>
        //         Return value from `transfer_spi`.
        //
        // Example:
        //     let result = spanda_hal::hal::transfer_spi(&self, _name, data);

        // Collect filtered entries into a new list.
        data.iter().map(|b| b ^ 0xff).collect()
    }

    fn read_uart(&self, name: &str) -> String {
        // Description:
        //     Read uart.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: String
        //         Return value from `read_uart`.
        //
        // Example:
        //     let result = spanda_hal::hal::read_uart(&self, name);

        // Call get on the current instance.
        self.uart_buffers.get(name).cloned().unwrap_or_default()
    }

    fn read_adc(&self, name: &str) -> f64 {
        // Description:
        //     Read adc.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: f64
        //         Return value from `read_adc`.
        //
        // Example:
        //     let result = spanda_hal::hal::read_adc(&self, name);

        // Call get on the current instance.
        self.adc_values.get(name).copied().unwrap_or(0.0)
    }

    fn set_pwm(&mut self, name: &str, duty_cycle: f64) {
        // Description:
        //     Set pwm.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     duty_cycle: f64
        //         Caller-supplied duty cycle.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_hal::hal::set_pwm(&mut self, name, duty_cycle);

        // Call pwm duty on the current instance.
        self.pwm_duty
            .insert(name.to_string(), duty_cycle.clamp(0.0, 1.0));
    }

    fn get_member(&self, name: &str) -> Option<HalMemberConfig> {
        // Description:
        //     Get member.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: Option<HalMemberConfig>
        //         Return value from `get_member`.
        //
        // Example:
        //     let result = spanda_hal::hal::get_member(&self, name);

        // Call get on the current instance.
        self.members.get(name).cloned()
    }

    fn list_members(&self) -> Vec<HalMemberConfig> {
        // Description:
        //     List members.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<HalMemberConfig>
        //         Return value from `list_members`.
        //
        // Example:
        //     let result = spanda_hal::hal::list_members(&self);

        // Collect filtered entries into a new list.
        self.members.values().cloned().collect()
    }
}

pub fn create_sim_hal() -> SimHalBackend {
    // Description:
    //     Create sim hal.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: SimHalBackend
    //         Return value from `create_sim_hal`.
    //
    // Example:
    //     let result = spanda_hal::hal::create_sim_hal();

    // Produce new as the result.
    SimHalBackend::new()
}

pub fn hal_member_from_decl(decl: &HalMemberDecl) -> HalMemberConfig {
    // Description:
    //     Hal member from decl.
    //
    // Inputs:
    //     decl: &HalMemberDecl
    //         Caller-supplied decl.
    //
    // Outputs:
    //     result: HalMemberConfig
    //         Return value from `hal_member_from_decl`.
    //
    // Example:
    //     let result = spanda_hal::hal::hal_member_from_decl(decl);

    // Match on decl and handle each case.
    match decl {
        HalMemberDecl::HalI2cDecl { name, address, .. } => HalMemberConfig::I2c {
            name: name.clone(),
            address: *address,
        },
        HalMemberDecl::HalSpiDecl {
            name, bus, cs_pin, ..
        } => HalMemberConfig::Spi {
            name: name.clone(),
            bus: *bus,
            cs_pin: *cs_pin,
        },
        HalMemberDecl::HalGpioDecl {
            name,
            direction,
            pin,
            ..
        } => HalMemberConfig::Gpio {
            name: name.clone(),
            pin: *pin,
            direction: *direction,
        },
        HalMemberDecl::HalPwmDecl {
            name,
            pin,
            frequency_hz,
            ..
        } => HalMemberConfig::Pwm {
            name: name.clone(),
            pin: *pin,
            frequency_hz: *frequency_hz,
        },
        HalMemberDecl::HalUartDecl {
            name, device, baud, ..
        } => HalMemberConfig::Uart {
            name: name.clone(),
            device: device.clone(),
            baud: *baud,
        },
        HalMemberDecl::HalAdcDecl { name, channel, .. } => HalMemberConfig::Adc {
            name: name.clone(),
            channel: *channel,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulates_i2c() {
        // Description:
        //     Simulates i2c.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_hal::hal::simulates_i2c();

        let mut hal = create_sim_hal();
        hal.configure(&[HalMemberConfig::I2c {
            name: "bus".to_string(),
            address: 104.0,
        }]);
        hal.write_i2c("bus", 0x10, &[0xab, 0xcd]);
        assert_eq!(hal.read_i2c("bus", 0x10, 2), vec![0xab, 0xcd]);
    }
}
