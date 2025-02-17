//! Multi-protocol synchronous serial engine utilities for FTDI devices.
#![doc(html_root_url = "https://docs.rs/ftdi-mpsse/0.1.0")]
#![deny(unsafe_code)]

use std::convert::From;
use std::result::Result;
use std::time::Duration;

/// MPSSE opcodes.
///
/// Exported for use by [`mpsse`] macro. May also be used for manual command array construction.
///
/// Data clocking MPSSE commands are broken out into separate enums for API ergonomics:
/// * [`ClockDataOut`]
/// * [`ClockBitsOut`]
/// * [`ClockDataIn`]
/// * [`ClockBitsIn`]
/// * [`ClockData`]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
pub enum MpsseCmd {
    /// Used by [`set_gpio_lower`][`MpsseCmdBuilder::set_gpio_lower`].
    SetDataBitsLowbyte = 0x80,
    /// Used by [`gpio_lower`][`MpsseCmdBuilder::gpio_lower`].
    GetDataBitsLowbyte = 0x81,
    /// Used by [`set_gpio_upper`][`MpsseCmdBuilder::set_gpio_upper`].
    SetDataBitsHighbyte = 0x82,
    /// Used by [`gpio_upper`][`MpsseCmdBuilder::gpio_upper`].
    GetDataBitsHighbyte = 0x83,
    /// Used by [`enable_loopback`][`MpsseCmdBuilder::enable_loopback`].
    EnableLoopback = 0x84,
    /// Used by [`disable_loopback`][`MpsseCmdBuilder::disable_loopback`].
    DisableLoopback = 0x85,
    /// Used by [`set_clock`][`MpsseCmdBuilder::set_clock`].
    SetClockFrequency = 0x86,
    /// Used by [`send_immediate`][`MpsseCmdBuilder::send_immediate`].
    SendImmediate = 0x87,
    /// Used by [`wait_on_io_high`][`MpsseCmdBuilder::wait_on_io_high`].
    WaitOnIOHigh = 0x88,
    /// Used by [`wait_on_io_low`][`MpsseCmdBuilder::wait_on_io_low`].
    WaitOnIOLow = 0x89,
    /// Used by [`set_clock`][`MpsseCmdBuilder::set_clock`].
    DisableClockDivide = 0x8A,
    /// Used by [`set_clock`][`MpsseCmdBuilder::set_clock`].
    EnableClockDivide = 0x8B,
    /// Used by [`enable_3phase_data_clocking`][`MpsseCmdBuilder::enable_3phase_data_clocking`].
    Enable3PhaseClocking = 0x8C,
    /// Used by [`disable_3phase_data_clocking`][`MpsseCmdBuilder::disable_3phase_data_clocking`].
    Disable3PhaseClocking = 0x8D,
    /// Used by [`disable_adaptive_data_clocking`][`MpsseCmdBuilder::disable_adaptive_data_clocking`].
    EnableAdaptiveClocking = 0x96,
    /// Used by [`enable_adaptive_data_clocking`][`MpsseCmdBuilder::enable_adaptive_data_clocking`].
    DisableAdaptiveClocking = 0x97,
    // EnableDriveOnlyZero = 0x9E,
}

/// Modes for clocking data out of the FTDI device.
///
/// This is an argument to the [`clock_data_out`] method.
///
/// [`clock_data_out`]: MpsseCmdBuilder::clock_data_out
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClockDataOut {
    /// Positive clock edge MSB first.
    ///
    /// The data is sent MSB first.
    ///
    /// The data will change to the next bit on the rising edge of the CLK pin.
    MsbPos = 0x10,
    /// Negative clock edge MSB first.
    ///
    /// The data is sent MSB first.
    ///
    /// The data will change to the next bit on the falling edge of the CLK pin.
    MsbNeg = 0x11,
    /// Positive clock edge LSB first.
    ///
    /// The first bit in will be the LSB of the first byte and so on.
    ///
    /// The data will change to the next bit on the rising edge of the CLK pin.
    LsbPos = 0x18,
    /// Negative clock edge LSB first.
    ///
    /// The first bit in will be the LSB of the first byte and so on.
    ///
    /// The data will change to the next bit on the falling edge of the CLK pin.
    LsbNeg = 0x19,
}

impl From<ClockDataOut> for u8 {
    fn from(value: ClockDataOut) -> u8 {
        value as u8
    }
}

/// Modes for clocking bits out of the FTDI device.
///
/// This is an argument to the [`clock_bits_out`] method.
///
/// [`clock_bits_out`]: MpsseCmdBuilder::clock_bits_out
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClockBitsOut {
    /// Positive clock edge MSB first.
    ///
    /// The data is sent MSB first (bit 7 first).
    ///
    /// The data will change to the next bit on the rising edge of the CLK pin.
    MsbPos = 0x12,
    /// Negative clock edge MSB first.
    ///
    /// The data is sent MSB first (bit 7 first).
    ///
    /// The data will change to the next bit on the falling edge of the CLK pin.
    MsbNeg = 0x13,
    /// Positive clock edge LSB first (bit 0 first).
    ///
    /// The first bit in will be the LSB of the first byte and so on.
    ///
    /// The data will change to the next bit on the rising edge of the CLK pin.
    LsbPos = 0x1A,
    /// Negative clock edge LSB first (bit 0 first).
    ///
    /// The first bit in will be the LSB of the first byte and so on.
    ///
    /// The data will change to the next bit on the falling edge of the CLK pin.
    LsbNeg = 0x1B,
}

impl From<ClockBitsOut> for u8 {
    fn from(value: ClockBitsOut) -> u8 {
        value as u8
    }
}

/// Modes for clocking data into the FTDI device.
///
/// This is an argument to the [`clock_data_in`] method.
///
/// [`clock_data_in`]: MpsseCmdBuilder::clock_data_in
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClockDataIn {
    /// Positive clock edge MSB first.
    ///
    /// The first bit in will be the MSB of the first byte and so on.
    ///
    /// The data will be sampled on the rising edge of the CLK pin.
    MsbPos = 0x20,
    /// Negative clock edge MSB first.
    ///
    /// The first bit in will be the MSB of the first byte and so on.
    ///
    /// The data will be sampled on the falling edge of the CLK pin.
    MsbNeg = 0x24,
    /// Positive clock edge LSB first.
    ///
    /// The first bit in will be the LSB of the first byte and so on.
    ///
    /// The data will be sampled on the rising edge of the CLK pin.
    LsbPos = 0x28,
    /// Negative clock edge LSB first.
    ///
    /// The first bit in will be the LSB of the first byte and so on.
    ///
    /// The data will be sampled on the falling edge of the CLK pin.
    LsbNeg = 0x2C,
}

impl From<ClockDataIn> for u8 {
    fn from(value: ClockDataIn) -> u8 {
        value as u8
    }
}

/// Modes for clocking data bits into the FTDI device.
///
/// This is an argument to the [`clock_bits_in`] method.
///
/// [`clock_bits_in`]: MpsseCmdBuilder::clock_bits_in
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClockBitsIn {
    /// Positive clock edge MSB first.
    ///
    /// The data will be shifted up so that the first bit in may not be in bit 7
    /// but from 6 downwards depending on the number of bits to shift
    /// (i.e. a length of 1 bit will have the data bit sampled in bit 0 of the
    /// byte sent back to the PC).
    ///
    /// The data will be sampled on the rising edge of the CLK pin.
    MsbPos = 0x22,
    /// Negative clock edge MSB first.
    ///
    /// The data will be shifted up so that the first bit in may not be in bit 7
    /// but from 6 downwards depending on the number of bits to shift
    /// (i.e. a length of 1 bit will have the data bit sampled in bit 0 of the
    /// byte sent back to the PC).
    ///
    /// The data will be sampled on the falling edge of the CLK pin.
    MsbNeg = 0x26,
    /// Positive clock edge LSB first.
    ///
    /// The data will be shifted down so that the first bit in may not be in bit
    /// 0 but from 1 upwards depending on the number of bits to shift
    /// (i.e. a length of 1 bit will have the data bit sampled in bit 7 of the
    /// byte sent back to the PC).
    ///
    /// The data will be sampled on the rising edge of the CLK pin.
    LsbPos = 0x2A,
    /// Negative clock edge LSB first.
    ///
    /// The data will be shifted down so that the first bit in may not be in bit
    /// 0 but from 1 upwards depending on the number of bits to shift
    /// (i.e. a length of 1 bit will have the data bit sampled in bit 7 of the
    /// byte sent back to the PC).
    ///
    /// The data will be sampled on the falling edge of the CLK pin.
    LsbNeg = 0x2E,
}

impl From<ClockBitsIn> for u8 {
    fn from(value: ClockBitsIn) -> u8 {
        value as u8
    }
}

/// Modes for clocking data in and out of the FTDI device.
///
/// This is an argument to the [`clock_data`] method.
///
/// [`clock_data`]: MpsseCmdBuilder::clock_data
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClockData {
    /// MSB first, data in on positive edge, data out on negative edge.
    MsbPosIn = 0x31,
    /// MSB first, data in on negative edge, data out on positive edge.
    MsbNegIn = 0x34,
    /// LSB first, data in on positive edge, data out on negative edge.
    LsbPosIn = 0x39,
    /// LSB first, data in on negative edge, data out on positive edge.
    LsbNegIn = 0x3C,
}

impl From<ClockData> for u8 {
    fn from(value: ClockData) -> u8 {
        value as u8
    }
}

/// Modes for clocking data bits in and out of the FTDI device.
///
/// This is an argument to the [`clock_bits`] method.
///
/// [`clock_bits`]: MpsseCmdBuilder::clock_bits
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClockBits {
    /// MSB first, data in on positive edge, data out on negative edge.
    MsbPosIn = 0x33,
    /// MSB first, data in on negative edge, data out on positive edge.
    MsbNegIn = 0x36,
    /// LSB first, data in on positive edge, data out on negative edge.
    LsbPosIn = 0x3B,
    /// LSB first, data in on negative edge, data out on positive edge.
    LsbNegIn = 0x3E,
}

impl From<ClockBits> for u8 {
    fn from(value: ClockBits) -> u8 {
        value as u8
    }
}

impl From<MpsseCmd> for u8 {
    fn from(value: MpsseCmd) -> Self {
        value as u8
    }
}

/// Modes for clocking bits out on TMS for JTAG mode.
///
/// This is an argument to the [`clock_tms_out`] method.
///
/// [`clock_tms_out`]: MpsseCmdBuilder::clock_tms_out
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClockTMSOut {
    /// LSB first, TMS out on positive edge
    PosEdge = 0x4A,
    /// LSB first, TMS out on negative edge
    NegEdge = 0x4B,
}

impl From<ClockTMSOut> for u8 {
    fn from(value: ClockTMSOut) -> u8 {
        value as u8
    }
}

/// Modes for clocking bits out on TMS for JTAG mode while reading TDO.
///
/// This is an argument to the [`clock_tms`] method.
///
/// [`clock_tms`]: MpsseCmdBuilder::clock_tms
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClockTMS {
    /// LSB first, TMS out on positive edge, TDO in on positive edge.
    PosTMSPosTDO = 0x6A,
    /// LSB first, TMS out on positive edge, TDO in on negative edge.
    PosTMSNegTDO = 0x6E,
    /// LSB first, TMS out on negative edge, TDO in on positive edge.
    NegTMSPosTDO = 0x6B,
    /// LSB first, TMS out on negative edge, TDO in on negative edge.
    NegTMSNegTDO = 0x6F,
}

impl From<ClockTMS> for u8 {
    fn from(value: ClockTMS) -> u8 {
        value as u8
    }
}

/// Initialization settings for the MPSSE.
///
/// Settings can be written to the device with the appropriate
/// implementation of [`init`] method.
///
/// [`init`]: MpsseCmdExecutor::init
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MpsseSettings {
    /// Reset the MPSSE on initialization.
    pub reset: bool,
    /// USB in transfer size in bytes.
    pub in_transfer_size: u32,
    /// Read timeout.
    pub read_timeout: Duration,
    /// Write timeout.
    pub write_timeout: Duration,
    /// Latency timer.
    pub latency_timer: Duration,
    /// Bitmode mask.
    ///
    /// * A bit value of `0` sets the corresponding pin to an input.
    /// * A bit value of `1` sets the corresponding pin to an output.
    pub mask: u8,
    /// Clock frequency.
    ///
    /// If `None`, then no frequency changes will be applied.
    pub clock_frequency: Option<u32>,
}

impl std::default::Default for MpsseSettings {
    fn default() -> Self {
        MpsseSettings {
            reset: true,
            in_transfer_size: 4096,
            read_timeout: Duration::from_secs(1),
            write_timeout: Duration::from_secs(1),
            latency_timer: Duration::from_millis(16),
            mask: 0x00,
            clock_frequency: None,
        }
    }
}

/// FTDI MPSSE configurator and executor
pub trait MpsseCmdExecutor {
    /// Error type
    type Error;

    /// Configure FTDI MPSSE mode
    fn init(&mut self, settings: &MpsseSettings) -> Result<(), Self::Error>;

    /// Execute MPSSE write command sequence
    fn send(&mut self, data: &[u8]) -> Result<(), Self::Error>;

    /// Execute MPSSE read command sequence
    fn recv(&mut self, data: &mut [u8]) -> Result<(), Self::Error>;

    /// Execute MPSSE command and read response
    fn xfer(&mut self, txdata: &[u8], rxdata: &mut [u8]) -> Result<(), Self::Error> {
        self.send(txdata)?;
        self.recv(rxdata)
    }
}

/// FTDI Multi-Protocol Synchronous Serial Engine (MPSSE) command builder.
///
/// For details about the MPSSE read the [FTDI MPSSE Basics].
///
/// This structure is a `Vec<u8>` that the methods push bytewise commands onto.
/// These commands can then be written to the device with the appropriate
/// implementations of [`send`] and [`xfer`] methods.
///
/// This is useful for creating commands that need to do multiple operations
/// quickly, since individual write calls can be expensive. For example,
/// this can be used to set a GPIO low and clock data out for SPI operations.
///
/// If dynamic command layout is not required, the [`mpsse`] macro can build
/// command `[u8; N]` arrays at compile-time.
///
/// [FTDI MPSSE Basics]: https://www.ftdichip.com/Support/Documents/AppNotes/AN_135_MPSSE_Basics.pdf
/// [`send`]: MpsseCmdExecutor::send
/// [`xfer`]: MpsseCmdExecutor::xfer
pub struct MpsseCmdBuilder(pub Vec<u8>);

impl MpsseCmdBuilder {
    /// Create a new command builder.
    ///
    /// # Example
    ///
    /// ```
    /// use ftdi_mpsse::MpsseCmdBuilder;
    ///
    /// MpsseCmdBuilder::new();
    /// ```
    pub const fn new() -> MpsseCmdBuilder {
        MpsseCmdBuilder(Vec::new())
    }

    /// Create a new command builder from a vector.
    ///
    /// # Example
    ///
    /// ```
    /// use ftdi_mpsse::MpsseCmdBuilder;
    ///
    /// MpsseCmdBuilder::with_vec(Vec::new());
    /// ```
    pub const fn with_vec(vec: Vec<u8>) -> MpsseCmdBuilder {
        MpsseCmdBuilder(vec)
    }

    /// Get the MPSSE command as a slice.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ftdi_mpsse::MpsseCmdBuilder;
    /// use libftd2xx::{Ft232h, FtdiCommon, FtdiMpsse};
    ///
    /// let cmd = MpsseCmdBuilder::new().enable_loopback();
    ///
    /// let mut ft = Ft232h::with_serial_number("FT5AVX6B")?;
    /// ft.initialize_mpsse_default()?;
    /// ft.set_clock(100_000);
    /// ft.write_all(cmd.as_slice())?;
    /// # Ok::<(), std::boxed::Box<dyn std::error::Error>>(())
    /// ```
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    /// Set the MPSSE clock frequency using provided
    /// divisor value and clock divider configuration.
    /// Both parameters are device dependent.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ftdi_mpsse::MpsseCmdBuilder;
    ///
    /// let cmd = MpsseCmdBuilder::new().set_clock(9, Some(false));
    ///
    /// ```
    pub fn set_clock(mut self, divisor: u32, clkdiv: Option<bool>) -> Self {
        match clkdiv {
            Some(true) => self.0.push(MpsseCmd::EnableClockDivide.into()),
            Some(false) => self.0.push(MpsseCmd::DisableClockDivide.into()),
            None => {}
        };

        self.0.push(MpsseCmd::SetClockFrequency.into());
        self.0.push((divisor & 0xFF) as u8);
        self.0.push(((divisor >> 8) & 0xFF) as u8);

        self
    }

    /// Enable the MPSSE loopback state.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ftdi_mpsse::MpsseCmdBuilder;
    /// use libftd2xx::{Ft232h, FtdiCommon, FtdiMpsse};
    ///
    /// let cmd = MpsseCmdBuilder::new().enable_loopback();
    ///
    /// let mut ft = Ft232h::with_serial_number("FT5AVX6B")?;
    /// ft.initialize_mpsse_default()?;
    /// ft.write_all(cmd.as_slice())?;
    /// # Ok::<(), std::boxed::Box<dyn std::error::Error>>(())
    /// ```
    pub fn enable_loopback(mut self) -> Self {
        self.0.push(MpsseCmd::EnableLoopback.into());
        self
    }

    /// Disable the MPSSE loopback state.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ftdi_mpsse::MpsseCmdBuilder;
    /// use libftd2xx::{Ft232h, FtdiCommon, FtdiMpsse};
    ///
    /// let cmd = MpsseCmdBuilder::new().disable_loopback();
    ///
    /// let mut ft = Ft232h::with_serial_number("FT5AVX6B")?;
    /// ft.initialize_mpsse_default()?;
    /// ft.write_all(cmd.as_slice())?;
    /// # Ok::<(), std::boxed::Box<dyn std::error::Error>>(())
    /// ```
    pub fn disable_loopback(mut self) -> Self {
        self.0.push(MpsseCmd::DisableLoopback.into());
        self
    }

    /// Disable 3 phase data clocking.
    ///
    /// This is only available on FTx232H devices.
    ///
    /// This will give a 2 stage data shift which is the default state.
    ///
    /// It will appears as:
    ///
    /// 1. Data setup for 1/2 clock period
    /// 2. Pulse clock for 1/2 clock period
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ftdi_mpsse::MpsseCmdBuilder;
    /// use libftd2xx::{Ft232h, FtdiCommon, FtdiMpsse};
    ///
    /// let cmd = MpsseCmdBuilder::new().disable_3phase_data_clocking();
    ///
    /// let mut ft = Ft232h::with_serial_number("FT5AVX6B")?;
    /// ft.initialize_mpsse_default()?;
    /// ft.write_all(cmd.as_slice())?;
    /// # Ok::<(), std::boxed::Box<dyn std::error::Error>>(())
    /// ```
    pub fn disable_3phase_data_clocking(mut self) -> Self {
        self.0.push(MpsseCmd::Disable3PhaseClocking.into());
        self
    }

    /// Enable 3 phase data clocking.
    ///
    /// This is only available on FTx232H devices.
    ///
    /// This will give a 3 stage data shift for the purposes of supporting
    /// interfaces such as I2C which need the data to be valid on both edges of
    /// the clock.
    ///
    /// It will appears as:
    ///
    /// 1. Data setup for 1/2 clock period
    /// 2. Pulse clock for 1/2 clock period
    /// 3. Data hold for 1/2 clock period
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ftdi_mpsse::MpsseCmdBuilder;
    /// use libftd2xx::{Ft232h, FtdiCommon, FtdiMpsse};
    ///
    /// let cmd = MpsseCmdBuilder::new().enable_3phase_data_clocking();
    ///
    /// let mut ft = Ft232h::with_serial_number("FT5AVX6B")?;
    /// ft.initialize_mpsse_default()?;
    /// ft.write_all(cmd.as_slice())?;
    /// # Ok::<(), std::boxed::Box<dyn std::error::Error>>(())
    /// ```
    pub fn enable_3phase_data_clocking(mut self) -> Self {
        self.0.push(MpsseCmd::Enable3PhaseClocking.into());
        self
    }

    /// Enable adaptive data clocking.
    ///
    /// This is only available on FTx232H devices.
    pub fn enable_adaptive_data_clocking(mut self) -> Self {
        self.0.push(MpsseCmd::EnableAdaptiveClocking.into());
        self
    }

    /// Enable adaptive data clocking.
    ///
    /// This is only available on FTx232H devices.
    pub fn disable_adaptive_data_clocking(mut self) -> Self {
        self.0.push(MpsseCmd::DisableAdaptiveClocking.into());
        self
    }

    /// Set the pin direction and state of the lower byte (0-7) GPIO pins on the
    /// MPSSE interface.
    ///
    /// The pins that this controls depends on the device.
    ///
    /// * On the FT232H this will control the AD0-AD7 pins.
    ///
    /// # Arguments
    ///
    /// * `state` - GPIO state mask, `0` is low (or input pin), `1` is high.
    /// * `direction` - GPIO direction mask, `0` is input, `1` is output.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ftdi_mpsse::MpsseCmdBuilder;
    /// use libftd2xx::{Ft232h, FtdiCommon, FtdiMpsse};
    ///
    /// let cmd = MpsseCmdBuilder::new()
    ///     .set_gpio_lower(0xFF, 0xFF)
    ///     .set_gpio_lower(0x00, 0xFF);
    ///
    /// let mut ft = Ft232h::with_serial_number("FT5AVX6B")?;
    /// ft.initialize_mpsse_default()?;
    /// ft.write_all(cmd.as_slice())?;
    /// # Ok::<(), std::boxed::Box<dyn std::error::Error>>(())
    /// ```
    pub fn set_gpio_lower(mut self, state: u8, direction: u8) -> Self {
        self.0
            .extend_from_slice(&[MpsseCmd::SetDataBitsLowbyte.into(), state, direction]);
        self
    }

    /// Set the pin direction and state of the upper byte (8-15) GPIO pins on
    /// the MPSSE interface.
    ///
    /// The pins that this controls depends on the device.
    /// This method may do nothing for some devices, such as the FT4232H that
    /// only have 8 pins per port.
    ///
    /// # Arguments
    ///
    /// * `state` - GPIO state mask, `0` is low (or input pin), `1` is high.
    /// * `direction` - GPIO direction mask, `0` is input, `1` is output.
    ///
    /// # FT232H Corner Case
    ///
    /// On the FT232H only CBUS5, CBUS6, CBUS8, and CBUS9 can be controlled.
    /// These pins confusingly map to the first four bits in the direction and
    /// state masks.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ftdi_mpsse::MpsseCmdBuilder;
    /// use libftd2xx::{Ft232h, FtdiCommon, FtdiMpsse};
    ///
    /// let cmd = MpsseCmdBuilder::new()
    ///     .set_gpio_upper(0xFF, 0xFF)
    ///     .set_gpio_upper(0x00, 0xFF);
    ///
    /// let mut ft = Ft232h::with_serial_number("FT5AVX6B")?;
    /// ft.initialize_mpsse_default()?;
    /// ft.write_all(cmd.as_slice())?;
    /// # Ok::<(), std::boxed::Box<dyn std::error::Error>>(())
    /// ```
    pub fn set_gpio_upper(mut self, state: u8, direction: u8) -> Self {
        self.0
            .extend_from_slice(&[MpsseCmd::SetDataBitsHighbyte.into(), state, direction]);
        self
    }

    /// Get the pin state state of the lower byte (0-7) GPIO pins on the MPSSE
    /// interface.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ftdi_mpsse::MpsseCmdBuilder;
    /// use libftd2xx::{Ft232h, FtdiCommon, FtdiMpsse};
    ///
    /// let cmd = MpsseCmdBuilder::new().gpio_lower().send_immediate();
    ///
    /// let mut ft = Ft232h::with_serial_number("FT5AVX6B")?;
    /// ft.initialize_mpsse_default()?;
    /// ft.write_all(cmd.as_slice())?;
    /// let mut buf: [u8; 1] = [0; 1];
    /// ft.read_all(&mut buf)?;
    /// println!("GPIO lower state: 0x{:02X}", buf[0]);
    /// # Ok::<(), std::boxed::Box<dyn std::error::Error>>(())
    /// ```
    pub fn gpio_lower(mut self) -> Self {
        self.0.push(MpsseCmd::GetDataBitsLowbyte.into());
        self
    }

    /// Get the pin state state of the upper byte (8-15) GPIO pins on the MPSSE
    /// interface.
    ///
    /// See [`set_gpio_upper`] for additional information about physical pin
    /// mappings.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ftdi_mpsse::MpsseCmdBuilder;
    /// use libftd2xx::{Ft232h, FtdiCommon, FtdiMpsse};
    ///
    /// let cmd = MpsseCmdBuilder::new().gpio_upper().send_immediate();
    ///
    /// let mut ft = Ft232h::with_serial_number("FT5AVX6B")?;
    /// ft.initialize_mpsse_default()?;
    /// ft.write_all(cmd.as_slice())?;
    /// let mut buf: [u8; 1] = [0; 1];
    /// ft.read_all(&mut buf)?;
    /// println!("GPIO upper state: 0x{:02X}", buf[0]);
    /// # Ok::<(), std::boxed::Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`set_gpio_upper`]: MpsseCmdBuilder::set_gpio_upper
    pub fn gpio_upper(mut self) -> Self {
        self.0.push(MpsseCmd::GetDataBitsHighbyte.into());
        self
    }

    /// Send the preceding commands immediately.
    ///
    /// # Example
    ///
    /// ```
    /// use ftdi_mpsse::MpsseCmdBuilder;
    ///
    /// let cmd = MpsseCmdBuilder::new()
    ///     .set_gpio_upper(0xFF, 0xFF)
    ///     .set_gpio_upper(0x00, 0xFF)
    ///     .send_immediate();
    /// ```
    pub fn send_immediate(mut self) -> Self {
        self.0.push(MpsseCmd::SendImmediate.into());
        self
    }

    /// Make controller wait until GPIOL1 or I/O1 is high before running further commands.
    ///
    /// # Example
    ///
    /// ```
    /// use ftdi_mpsse::{ClockData, MpsseCmdBuilder};
    ///
    /// // Assume a "chip ready" signal is connected to GPIOL1. This signal is pulled high
    /// // shortly after AD3 (chip select) is pulled low. Data will not be clocked out until
    /// // the chip is ready.
    /// let cmd = MpsseCmdBuilder::new()
    ///     .set_gpio_lower(0x0, 0xb)
    ///     .wait_on_io_high()
    ///     .clock_data(ClockData::MsbPosIn, &[0x12, 0x34, 0x56])
    ///     .set_gpio_lower(0x8, 0xb)
    ///     .send_immediate();
    /// ```
    pub fn wait_on_io_high(mut self) -> Self {
        self.0.push(MpsseCmd::WaitOnIOHigh.into());
        self
    }

    /// Make controller wait until GPIOL1 or I/O1 is low before running further commands.
    ///
    /// # Example
    ///
    /// ```
    /// use ftdi_mpsse::{ClockData, MpsseCmdBuilder};
    ///
    /// // Assume a "chip ready" signal is connected to GPIOL1. This signal is pulled low
    /// // shortly after AD3 (chip select) is pulled low. Data will not be clocked out until
    /// // the chip is ready.
    /// let cmd = MpsseCmdBuilder::new()
    ///     .set_gpio_lower(0x0, 0xb)
    ///     .wait_on_io_low()
    ///     .clock_data(ClockData::MsbPosIn, &[0x12, 0x34, 0x56])
    ///     .set_gpio_lower(0x8, 0xb)
    ///     .send_immediate();
    /// ```
    pub fn wait_on_io_low(mut self) -> Self {
        self.0.push(MpsseCmd::WaitOnIOLow.into());
        self
    }

    /// Clock data out.
    ///
    /// This will clock out bytes on TDI/DO.
    /// No data is clocked into the device on TDO/DI.
    ///
    /// This will panic for data lengths greater than `u16::MAX + 1`.
    pub fn clock_data_out(mut self, mode: ClockDataOut, data: &[u8]) -> Self {
        let mut len = data.len();
        assert!(len <= 65536, "data length cannot exceed u16::MAX + 1");
        if len == 0 {
            return self;
        }
        len -= 1;
        self.0
            .extend_from_slice(&[mode.into(), (len & 0xFF) as u8, ((len >> 8) & 0xFF) as u8]);
        self.0.extend_from_slice(data);
        self
    }

    /// Clock data in.
    ///
    /// This will clock in bytes on TDO/DI.
    /// No data is clocked out of the device on TDI/DO.
    ///
    /// # Arguments
    ///
    /// * `mode` - Data clocking mode.
    /// * `len` - Number of bytes to clock in.
    ///           This will panic for values greater than `u16::MAX + 1`.
    pub fn clock_data_in(mut self, mode: ClockDataIn, mut len: usize) -> Self {
        assert!(len <= 65536, "data length cannot exceed u16::MAX + 1");
        if len == 0 {
            return self;
        }
        len -= 1;
        self.0
            .extend_from_slice(&[mode.into(), (len & 0xFF) as u8, ((len >> 8) & 0xFF) as u8]);
        self
    }

    /// Clock data in and out simultaneously.
    ///
    /// This will panic for data lengths greater than `u16::MAX + 1`.
    pub fn clock_data(mut self, mode: ClockData, data: &[u8]) -> Self {
        let mut len = data.len();
        assert!(len <= 65536, "data length cannot exceed u16::MAX + 1");
        if len == 0 {
            return self;
        }
        len -= 1;
        self.0
            .extend_from_slice(&[mode.into(), (len & 0xFF) as u8, ((len >> 8) & 0xFF) as u8]);
        self.0.extend_from_slice(data);
        self
    }

    /// Clock data bits out.
    ///
    /// # Arguments
    ///
    /// * `mode` - Bit clocking mode.
    /// * `data` - Data bits.
    /// * `len` - Number of bits to clock out.
    ///           This will panic for values greater than 8.
    pub fn clock_bits_out(mut self, mode: ClockBitsOut, data: u8, mut len: u8) -> Self {
        assert!(len <= 8, "data length cannot exceed 8");
        if len == 0 {
            return self;
        }
        len -= 1;
        self.0.extend_from_slice(&[mode.into(), len, data]);
        self
    }

    /// Clock data bits in.
    ///
    /// # Arguments
    ///
    /// * `mode` - Bit clocking mode.
    /// * `len` - Number of bits to clock in.
    ///           This will panic for values greater than 8.
    pub fn clock_bits_in(mut self, mode: ClockBitsIn, mut len: u8) -> Self {
        assert!(len <= 8, "data length cannot exceed 8");
        if len == 0 {
            return self;
        }
        len -= 1;
        self.0.extend_from_slice(&[mode.into(), len]);
        self
    }

    /// Clock data bits in and out simultaneously.
    ///
    /// # Arguments
    ///
    /// * `mode` - Bit clocking mode.
    /// * `len` - Number of bits to clock in.
    ///           This will panic for values greater than 8.
    pub fn clock_bits(mut self, mode: ClockBits, data: u8, mut len: u8) -> Self {
        assert!(len <= 8, "data length cannot exceed 8");
        if len == 0 {
            return self;
        }
        len -= 1;
        self.0.extend_from_slice(&[mode.into(), len, data]);
        self
    }

    /// Clock TMS bits out.
    ///
    /// # Arguments
    ///
    /// * `mode` - TMS clocking mode.
    /// * `data` - TMS bits.
    /// * `tdi` - Value to place on TDI while clocking.
    /// * `len` - Number of bits to clock out.
    ///           This will panic for values greater than 7.
    pub fn clock_tms_out(
        mut self,
        mode: ClockTMSOut,
        mut data: u8,
        tdi: bool,
        mut len: u8,
    ) -> Self {
        assert!(len <= 7, "data length cannot exceed 7");
        if len == 0 {
            return self;
        }
        len -= 1;
        if tdi {
            data |= 0x80;
        }
        self.0.extend_from_slice(&[mode.into(), len, data]);
        self
    }

    /// Clock TMS bits out while clocking TDO bits in.
    ///
    /// # Arguments
    ///
    /// * `mode` - TMS clocking mode.
    /// * `data` - TMS bits.
    /// * `tdi` - Value to place on TDI while clocking.
    /// * `len` - Number of bits to clock out.
    ///           This will panic for values greater than 7.
    pub fn clock_tms(mut self, mode: ClockTMS, mut data: u8, tdi: bool, mut len: u8) -> Self {
        assert!(len <= 7, "data length cannot exceed 7");
        if len == 0 {
            return self;
        }
        len -= 1;
        if tdi {
            data |= 0x80;
        }
        self.0.extend_from_slice(&[mode.into(), len, data]);
        self
    }
}

/// Construct an MPSSE command array at compile-time.
///
/// Alternative to [`MpsseCmdBuilder`]. Parses a specialized grammar that gathers MPSSE commands
/// into pseudo-statements contained within zero or more assigned blocks. The pseudo-assignment
/// syntax of each block creates a fixed-length `[u8; N]` array that is bound with `let` or
/// `const`[^const_note].
///
/// [^const_note]: In `const` bindings, all values used as command parameters and data must be const.
///
/// # Syntax
///
/// ```compile_fail
/// mpsse! { let command_data = { command1(); command2(); /* ... */ commandN(); }; }
/// ```
/// or
/// ```compile_fail
/// mpsse! { let (command_data, READ_LEN) = { command1(); command2(); /* ... */ commandN(); }; }
/// ```
/// The second form provides the caller with a constant size value of the expected data length to
/// read after writing the commands to the device.
///
/// # Commands
///
/// * [`enable_loopback()`][`MpsseCmdBuilder::enable_loopback`]
/// * [`disable_loopback()`][`MpsseCmdBuilder::disable_loopback`]
/// * [`enable_3phase_data_clocking()`][`MpsseCmdBuilder::enable_3phase_data_clocking`]
/// * [`disable_3phase_data_clocking()`][`MpsseCmdBuilder::disable_3phase_data_clocking`]
/// * [`set_gpio_lower(state: u8, direction: u8)`][`MpsseCmdBuilder::set_gpio_lower`]
/// * [`set_gpio_upper(state: u8, direction: u8)`][`MpsseCmdBuilder::set_gpio_upper`]
/// * [`gpio_lower() -> usize`][`MpsseCmdBuilder::gpio_lower`]
/// * [`gpio_upper() -> usize`][`MpsseCmdBuilder::gpio_upper`]
/// * [`send_immediate()`][`MpsseCmdBuilder::send_immediate`]
/// * [`wait_on_io_high()`][`MpsseCmdBuilder::wait_on_io_high`]
/// * [`wait_on_io_low()`][`MpsseCmdBuilder::wait_on_io_low`]
/// * [`clock_data_out(mode: ClockDataOut, data: [u8])`][`MpsseCmdBuilder::clock_data_out`]
/// * [`clock_data_in(mode: ClockDataIn, len: u16) -> std::ops::Range<usize>`][`MpsseCmdBuilder::clock_data_in`]
/// * [`clock_data(mode: ClockData, data: [u8]) -> std::ops::Range<usize>`][`MpsseCmdBuilder::clock_data`]
/// * [`clock_bits_out(mode: ClockBitsOut, data: u8, len: u8)`][`MpsseCmdBuilder::clock_bits_out`]
/// * [`clock_bits_in(mode: ClockBitsIn, len: u8) -> usize`][`MpsseCmdBuilder::clock_bits_in`]
/// * [`clock_bits(mode: ClockBits, data: u8, len: u8) -> usize`][`MpsseCmdBuilder::clock_bits`]
/// * [`clock_tms_out(mode: ClockTMSOut, data: u8, tdi: bool, len: u8)`][`MpsseCmdBuilder::clock_tms_out`]
/// * [`clock_tms(mode: ClockTMS, data: u8, tdi: bool, len: u8) -> usize`][`MpsseCmdBuilder::clock_tms`]
///
/// Command pseudo-statements that read data from the device may optionally have the form:
/// ```
/// # use ftdi_mpsse::{mpsse, ClockDataIn};
/// mpsse! {
///     // command_data and DATA_IN_RANGE are both declared in the scope of the macro expansion.
///     let command_data = {
///         const DATA_IN_RANGE = clock_data_in(ClockDataIn::MsbNeg, 3);
///     };
/// }
/// ```
/// This provides a constant [`Range`][`std::ops::Range`] or [`usize`] index value that may be used
/// to subscript the data read from the device.
///
/// `clock_data` and `clock_data_out` require that the second argument is a fixed-length, square
/// bracketed list of `u8` values. Compile-time limitations make arbitrary array concatenation or
/// coercion infeasible.
///
/// # Asserts
///
/// For `let` bindings, the standard [`assert`] macro is used for validating parameter size inputs.
/// For `const` bindings, [`const_assert`][`static_assertions::const_assert`] is used instead.
///
/// `const_assert` lacks the ability to provide meaningful compile errors, so it may be useful
/// to temporarily use a `let` binding within function scope to diagnose failing macro expansions.
///
/// # User Abstractions
///
/// With macro shadowing, it is possible to extend the macro with additional rules for abstract,
/// device-specific commands.
///
/// Comments within the implementation of this macro contain hints on how to implement these rules.
///
/// For example, a SPI device typically delineates transfers with the CS line. Fundamental
/// commands like `cs_high` and `cs_low` can be implemented this way, along with other
/// device-specific abstractions.
///
/// ```
/// # use ftdi_mpsse::mpsse;
/// macro_rules! mpsse {
///     // Practical abstraction of CS line for SPI devices.
///     ($passthru:tt {cs_low(); $($tail:tt)*} -> [$($out:tt)*]) => {
///         mpsse!($passthru {
///             set_gpio_lower(0x0, 0xb);
///             $($tail)*
///         } -> [$($out)*]);
///     };
///     ($passthru:tt {cs_high(); $($tail:tt)*} -> [$($out:tt)*]) => {
///         mpsse!($passthru {
///             set_gpio_lower(0x8, 0xb);
///             $($tail)*
///         } -> [$($out)*]);
///     };
///
///     // Hypothetical device-specific command. Leverages both user and libftd2xx commands.
///     ($passthru:tt
///      {const $idx_id:ident = command_42([$($data:expr),* $(,)*]); $($tail:tt)*} ->
///      [$($out:tt)*]) => {
///         mpsse!($passthru {
///             cs_low();
///             const $idx_id = clock_data(::ftdi_mpsse::ClockData::MsbPosIn, [0x42, $($data,)*]);
///             cs_high();
///             $($tail)*
///         } -> [$($out)*]);
///     };
///
///     // Everything else handled by libftd2xx crate implementation.
///     ($($tokens:tt)*) => {
///         ::ftdi_mpsse::mpsse!($($tokens)*);
///     };
/// }
///
/// mpsse! {
///     const (COMMAND_DATA, READ_LEN) = {
///         wait_on_io_high();
///         const COMMAND_42_RESULT_RANGE = command_42([11, 22, 33]);
///         send_immediate();
///     };
/// }
/// ```
///
/// # Example
///
/// ```no_run
/// use ftdi_mpsse::{mpsse, ClockDataIn, ClockDataOut};
/// use libftd2xx::{Ft232h, FtdiCommon, FtdiMpsse};
///
/// mpsse! {
///     const (COMMAND_DATA, READ_LEN) = {
///         set_gpio_lower(0xFA, 0xFB);
///         set_gpio_lower(0xF2, 0xFB);
///         clock_data_out(ClockDataOut::MsbNeg, [0x12, 0x34, 0x56]);
///         const DATA_IN_RANGE = clock_data_in(ClockDataIn::MsbNeg, 3);
///         set_gpio_lower(0xFA, 0xFB);
///         send_immediate();
///     };
/// }
///
/// let mut ft = Ft232h::with_serial_number("FT5AVX6B")?;
/// ft.initialize_mpsse_default()?;
/// ft.write_all(&COMMAND_DATA)?;
/// let mut buf: [u8; READ_LEN] = [0; READ_LEN];
/// ft.read_all(&mut buf)?;
/// println!("Data slice in: {:?}", &buf[DATA_IN_RANGE]);
/// # Ok::<(), std::boxed::Box<dyn std::error::Error>>(())
/// ```
#[macro_export]
macro_rules! mpsse {
    // Replacement method for counting comma-separated expressions.
    // https://danielkeep.github.io/tlborm/book/blk-counting.html#repetition-with-replacement
    (@replace_expr $_t:tt $sub:expr) => {$sub};
    (@count_elements $($tts:expr),* $(,)*) => {(0usize $(+ mpsse!(@replace_expr $tts 1usize))*)};

    // Assert that is selectively compile-time depending on let vs. const expansion.
    //
    // Unfortunately, the compile-time error is not very helpful due to the lack of message and
    // macro depth, but still ensures safe command construction.
    //
    // Temporarily running a let expansion can be helpful to diagnose errors.
    (@assert ((let, $_user_passthru:tt), $_read_len:expr), $e:expr, $msg:expr) => {
        ::std::assert!($e, $msg);
    };
    (@assert ((const, $_user_passthru:tt), $_read_len:expr), $e:expr, $_msg:expr) => {
        ::static_assertions::const_assert!($e);
    };

    // Unit rule
    () => {};

    // let command_data = { command1(); command2(); ... commandN(); };
    (let $id:ident = {$($commands:tt)*}; $($tail:tt)*) => {
        mpsse!(((let, ($id, _)), 0) {$($commands)*} -> []);
        mpsse!($($tail)*);
    };

    // const COMMAND_DATA = { command1(); command2(); ... commandN(); };
    (const $id:ident = {$($commands:tt)*}; $($tail:tt)*) => {
        mpsse!(((const, ($id, _)), 0) {$($commands)*} -> []);
        mpsse!($($tail)*);
    };

    // let (command_data, READ_LEN) = { command1(); command2(); ... commandN(); };
    (let ($id:ident, $read_len_id:ident) = {$($commands:tt)*}; $($tail:tt)*) => {
        mpsse!(((let, ($id, $read_len_id)), 0) {$($commands)*} -> []);
        mpsse!($($tail)*);
    };

    // const (COMMAND_DATA, READ_LEN) = { command1(); command2(); ... commandN(); };
    (const ($id:ident, $read_len_id:ident) = {$($commands:tt)*}; $($tail:tt)*) => {
        mpsse!(((const, ($id, $read_len_id)), 0) {$($commands)*} -> []);
        mpsse!($($tail)*);
    };

    // Rules generally follow a structure based on three root token trees:
    // (<passthru>) {<input>} -> [<output>]
    //
    // "Statements" are recursively shifted off the front of the input and the resulting u8 tokens
    // are appended to the output. Recursion ends when the input token tree is empty.
    //
    // Rules have the following form:
    // ($passthru:tt {<FUNCTION NAME>(); $($tail:tt)*} -> [$($out:tt)*])
    //
    // For functions that perform data reads, cumulative read_len can be accessed with this form:
    // (($passthru:tt, $read_len:tt) {<FUNCTION NAME>(); $($tail:tt)*} -> [$($out:tt)*])
    //
    // Additionally, the following form is used to provide the invoker with a usize index or
    // range to later access a specific data read `const READ_INDEX = <FUNCTION NAME>();`:
    // (($passthru:tt, $read_len:tt) {const $idx_id:ident = <FUNCTION NAME>(); $($tail:tt)*} -> [$($out:tt)*])

    ($passthru:tt {enable_loopback(); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!($passthru {$($tail)*} -> [$($out)* $crate::MpsseCmd::EnableLoopback as u8,]);
    };
    ($passthru:tt {disable_loopback(); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!($passthru {$($tail)*} -> [$($out)* $crate::MpsseCmd::DisableLoopback as u8,]);
    };
    ($passthru:tt {enable_3phase_data_clocking(); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!($passthru {$($tail)*} -> [$($out)* $crate::MpsseCmd::Enable3PhaseClocking as u8,]);
    };
    ($passthru:tt {disable_3phase_data_clocking(); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!($passthru {$($tail)*} -> [$($out)* $crate::MpsseCmd::Disable3PhaseClocking as u8,]);
    };
    ($passthru:tt {enable_adaptive_data_clocking(); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!($passthru {$($tail)*} -> [$($out)* $crate::MpsseCmd::EnableDataClocking as u8,]);
    };
    ($passthru:tt {disable_adaptive_data_clocking(); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!($passthru {$($tail)*} -> [$($out)* $crate::MpsseCmd::DisableDataClocking as u8,]);
    };
    ($passthru:tt {set_gpio_lower($state:expr, $direction:expr); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!($passthru {$($tail)*} -> [$($out)* $crate::MpsseCmd::SetDataBitsLowbyte as u8, $state as u8, $direction as u8,]);
    };
    ($passthru:tt {set_gpio_upper($state:expr, $direction:expr); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!($passthru {$($tail)*} -> [$($out)* $crate::MpsseCmd::SetDataBitsHighbyte as u8, $state as u8, $direction as u8,]);
    };
    (($passthru:tt, $read_len:tt) {gpio_lower(); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!(($passthru, ($read_len + 1)) {$($tail)*} -> [$($out)* $crate::MpsseCmd::GetDataBitsLowbyte as u8,]);
    };
    (($passthru:tt, $read_len:tt) {const $idx_id:ident = gpio_lower(); $($tail:tt)*} -> [$($out:tt)*]) => {
        const $idx_id: usize = $read_len;
        mpsse!(($passthru, $read_len) {gpio_lower(); $($tail)*} -> [$($out)*]);
    };
    (($passthru:tt, $read_len:tt) {gpio_upper(); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!(($passthru, ($read_len + 1)) {$($tail)*} -> [$($out)* $crate::MpsseCmd::GetDataBitsHighbyte as u8,]);
    };
    (($passthru:tt, $read_len:tt) {const $idx_id:ident = gpio_upper(); $($tail:tt)*} -> [$($out:tt)*]) => {
        const $idx_id: usize = $read_len;
        mpsse!(($passthru, $read_len) {gpio_upper(); $($tail)*} -> [$($out)*]);
    };
    ($passthru:tt {send_immediate(); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!($passthru {$($tail)*} -> [$($out)* $crate::MpsseCmd::SendImmediate as u8,]);
    };
    ($passthru:tt {wait_on_io_high(); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!($passthru {$($tail)*} -> [$($out)* $crate::MpsseCmd::WaitOnIOHigh as u8,]);
    };
    ($passthru:tt {wait_on_io_low(); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!($passthru {$($tail)*} -> [$($out)* $crate::MpsseCmd::WaitOnIOLow as u8,]);
    };
    ($passthru:tt {clock_data_out($mode:expr, [$($data:expr),* $(,)*]); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!(@assert $passthru, (mpsse!(@count_elements $($data,)*) as usize > 0_usize && mpsse!(@count_elements $($data,)*) as usize <= 65536_usize), "data length must be in 1..=(u16::MAX + 1)");
        mpsse!($passthru {$($tail)*} -> [$($out)* $mode as $crate::ClockDataOut as u8,
        ((mpsse!(@count_elements $($data,)*) - 1) & 0xFF_usize) as u8,
        (((mpsse!(@count_elements $($data,)*) - 1) >> 8) & 0xFF_usize) as u8,
        $($data as u8,)*]);
    };
    (($passthru:tt, $read_len:tt) {clock_data_in($mode:expr, $len:expr); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!(@assert ($passthru, $read_len), (($len) as usize > 0_usize && ($len) as usize <= 65536_usize), "data length must be in 1..=(u16::MAX + 1)");
        mpsse!(($passthru, ($read_len + ($len))) {$($tail)*} -> [$($out)* $mode as $crate::ClockDataIn as u8,
        ((($len) - 1) & 0xFF_usize) as u8,
        (((($len) - 1) >> 8) & 0xFF_usize) as u8,]);
    };
    (($passthru:tt, $read_len:tt) {const $range_id:ident = clock_data_in($mode:expr, $len:expr); $($tail:tt)*} -> [$($out:tt)*]) => {
        const $range_id: ::std::ops::Range<usize> = $read_len..$read_len + ($len);
        mpsse!(($passthru, $read_len) {clock_data_in($mode, $len); $($tail)*} -> [$($out)*]);
    };
    (($passthru:tt, $read_len:tt) {clock_data($mode:expr, [$($data:expr),* $(,)*]); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!(@assert ($passthru, $read_len), (mpsse!(@count_elements $($data,)*) as usize > 0_usize && mpsse!(@count_elements $($data,)*) as usize <= 65536_usize), "data length must be in 1..=(u16::MAX + 1)");
        mpsse!(($passthru, ($read_len + mpsse!(@count_elements $($data,)*))) {$($tail)*} -> [$($out)* $mode as $crate::ClockData as u8,
        ((mpsse!(@count_elements $($data,)*) - 1) & 0xFF_usize) as u8,
        (((mpsse!(@count_elements $($data,)*) - 1) >> 8) & 0xFF_usize) as u8,
        $($data as u8,)*]);
    };
    (($passthru:tt, $read_len:tt) {const $range_id:ident = clock_data($mode:expr, [$($data:expr),* $(,)*]); $($tail:tt)*} -> [$($out:tt)*]) => {
        const $range_id: ::std::ops::Range<usize> = $read_len..$read_len + mpsse!(@count_elements $($data,)*);
        mpsse!(($passthru, $read_len) {clock_data($mode, [$($data,)*]); $($tail)*} -> [$($out)*]);
    };
    ($passthru:tt {clock_bits_out($mode:expr, $data:expr, $len:expr); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!(@assert $passthru, ($len as u8 > 0_u8 && $len as u8 <= 8_u8), "data length must be in 1..=8");
        mpsse!($passthru {$($tail)*} -> [$($out)* $mode as $crate::ClockBitsOut as u8, (($len) - 1) as u8, $data as u8,]);
    };
    (($passthru:tt, $read_len:tt) {clock_bits_in($mode:expr, $len:expr); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!(@assert ($passthru, $read_len), ($len as u8 > 0_u8 && $len as u8 <= 8_u8), "data length must be in 1..=8");
        mpsse!(($passthru, ($read_len + 1)) {$($tail)*} -> [$($out)* $mode as $crate::ClockBitsIn as u8, (($len) - 1) as u8,]);
    };
    (($passthru:tt, $read_len:tt) {const $idx_id:ident = clock_bits_in($mode:expr, $len:expr); $($tail:tt)*} -> [$($out:tt)*]) => {
        const $idx_id: usize = $read_len;
        mpsse!(($passthru, $read_len) {clock_bits_in($mode, $len); $($tail)*} -> [$($out)*]);
    };
    (($passthru:tt, $read_len:tt) {clock_bits($mode:expr, $data:expr, $len:expr); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!(@assert ($passthru, $read_len), ($len as u8 > 0_u8 && $len as u8 <= 8_u8), "data length must be in 1..=8");
        mpsse!(($passthru, ($read_len + 1)) {$($tail)*} -> [$($out)* $mode as $crate::ClockBits as u8, (($len) - 1) as u8, $data as u8,]);
    };
    (($passthru:tt, $read_len:tt) {const $idx_id:ident = clock_bits($mode:expr, $data:expr, $len:expr); $($tail:tt)*} -> [$($out:tt)*]) => {
        const $idx_id: usize = $read_len;
        mpsse!(($passthru, $read_len) {clock_bits($mode, $data, $len); $($tail)*} -> [$($out)*]);
    };
    ($passthru:tt {clock_tms_out($mode:expr, $data:expr, $tdi:expr, $len:expr); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!(@assert $passthru, ($len as u8 > 0_u8 && $len as u8 <= 7_u8), "data length must be in 1..=7");
        mpsse!($passthru {$($tail)*} -> [$($out)* $mode as $crate::ClockTMSOut as u8, (($len) - 1) as u8, ($data as u8) | if $tdi { 0x80 } else { 0 },]);
    };
    (($passthru:tt, $read_len:tt) {clock_tms($mode:expr, $data:expr, $tdi:expr, $len:expr); $($tail:tt)*} -> [$($out:tt)*]) => {
        mpsse!(@assert ($passthru, $read_len), ($len as u8 > 0_u8 && $len as u8 <= 7_u8), "data length must be in 1..=7");
        mpsse!(($passthru, ($read_len + 1)) {$($tail)*} -> [$($out)* $mode as $crate::ClockTMS as u8, (($len) - 1) as u8, ($data as u8) | if $tdi { 0x80 } else { 0 },]);
    };
    (($passthru:tt, $read_len:tt) {const $idx_id:ident = clock_tms($mode:expr, $data:expr, $tdi:expr, $len:expr); $($tail:tt)*} -> [$($out:tt)*]) => {
        const $idx_id: usize = $read_len;
        mpsse!(($passthru, $read_len) {clock_tms($mode, $data, $tdi, $len); $($tail)*} -> [$($out)*]);
    };

    // Emit command_data
    ((($const_let:tt, ($id:tt, _)), $read_len:expr) {} -> [$($out:tt)*]) => {
        $const_let $id: [u8; mpsse!(@count_elements $($out)*)] = [$($out)*];
    };

    // Emit command_data, READ_LEN
    ((($const_let:tt, ($id:tt, $read_len_id:tt)), $read_len:expr) {} -> [$($out:tt)*]) => {
        $const_let $id: [u8; mpsse!(@count_elements $($out)*)] = [$($out)*];
        const $read_len_id: usize = $read_len;
    };
}
