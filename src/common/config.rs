use crate::{
    mode,
    register_access::{BitFlags, Register},
    Error, GScale, Mma8x5x, ReadMode,
};
use embedded_hal::blocking::i2c;

impl<E, I2C, IC> Mma8x5x<I2C, IC, mode::Standby>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Set G scale: +/-2g, +/-4g, +/-8g
    pub fn set_scale(&mut self, scale: GScale) -> Result<(), Error<E>> {
        let config = match scale {
            GScale::G2 => self
                .xyz_data_cfg
                .with_low(BitFlags::FS1)
                .with_low(BitFlags::FS0),
            GScale::G4 => self
                .xyz_data_cfg
                .with_low(BitFlags::FS1)
                .with_high(BitFlags::FS0),
            GScale::G8 => self
                .xyz_data_cfg
                .with_high(BitFlags::FS1)
                .with_low(BitFlags::FS0),
        };
        self.write_reg(Register::XYZ_DATA_CFG, config.bits)?;
        self.xyz_data_cfg = config;
        Ok(())
    }

    /// Set read mode (Normal/Fast)
    pub fn set_read_mode(&mut self, mode: ReadMode) -> Result<(), Error<E>> {
        let config = match mode {
            ReadMode::Normal => self.ctrl_reg1.with_low(BitFlags::F_READ),
            ReadMode::Fast => self.ctrl_reg1.with_high(BitFlags::F_READ),
        };
        self.write_reg(Register::CTRL_REG1, config.bits)?;
        self.ctrl_reg1 = config;
        Ok(())
    }

    /// Set offset correction.
    ///
    /// The resolution is 1.96/LSB. The offset compensation range is +/-250mg.
    pub fn set_offset_correction(&mut self, x: i8, y: i8, z: i8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::OFF_X, x as u8, y as u8, z as u8])
            .map_err(Error::I2C)
    }
}