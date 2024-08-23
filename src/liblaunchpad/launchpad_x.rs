//! This module contains useful functions for interacting with the Launchpad X.
const SYSEX_HEADER: [u8; 6] = [0xF0, 0x00, 0x20, 0x29, 0x02, 0x0C];

const SYSEX_READBACK_FOOTER: [u8; 2] = [0xF7];

///Enum for Launchpad X commands
#[derive(Debug)]
enum LaunchpadXCommand {
    SelectLayout = 0x00,
    DawFaderSetup = 0x01,
    LedLighting = 0x03,
    VelocityCurve = 0x04,
    TextScrolling = 0x07,
    BrightnessLevel = 0x08,
    LedSleep = 0x09,
    LedFeedback = 0x0A,
    AftertouchConfig = 0x0B,
    FaderVelocity = 0x0D,
    ProgrammerLiveToggle = 0x0E,
    DawNoteDrumRack = 0x0F,
    DawStandalone = 0x10,
    ClearDawState = 0x12,
    DrumRackPosition = 0x13,
    SessionColor = 0x14,
    NoteModeScaleChromatic = 0x15,
    NoteModeConfiguration = 0x16,
    DawNoteActiveColor = 0x17,
}

struct LaunchpadX {
    conn_out: Some(MidiOutputConnection),
}

impl LaunchpadX {
    pub fn new(conn_out: MidiOutputConnection) -> Self {
        Self { None }
    }

    /// Light up a pad on the Launchpad X with a specific color and lighting type.
    fn light_pad(&mut self, pad_index: u8, color: u8, light_type: u8) -> Result<(), Box<dyn Error>> {
        check_pad_index(pad_index)?;
        check_color_value(color)?;
        check_light_type(light_type)?;
        let sysex_msg = [
            SYSEX_HEADER,
            LaunchpadXCommand::LedLighting, // Command: Set LED
            pad_index, // Pad index
            color, // Color value
            SYSEX_READBACK_FOOTER
        ].concat();
        self.conn_out.send(&sysex_msg)?;
        Ok(())
    }

    /// Checks if the pad index is valid. Erros if the pad index is not between 11 and 99 or is a multiple of 10.
    fn check_pad_index(pad_index: u8) -> Result<(), Box<dyn Error>> {
        if pad_index > 99 || pad_index < 11 || pad_index % 10 = 0  {
            return Err("Invalid pad index".into());
        }
        Ok(())
    }

    /// Checks if the color value is valid. Errors if the color value is greater than 127.
    fn check_color_value(color_value: u8) -> Result<(), Box<dyn Error>> {
        if color_value > 127 {
            return Err("Invalid color value".into());
        }
        Ok(())
    }

    fn check_light_type(light_type: u8) -> Result<(), Box<dyn Error>> {
        if light_type > 3 {
            return Err("Invalid light type".into());
        }
        Ok(())
    }
}   