use super::*;
use crate::as_bytes;

mod mic_core_fir;
use mic_core_fir::*;

type Coeffs = [i16; 128];
pub struct MicCore<'a> {
    mics: &'a MicArray<'a>,
    fir_coeff: Coeffs,
}

impl<'a> MicCore<'a> {
    /// Return an instance of MicCore.
    pub fn new(mics: &'a MicArray) -> Self {
        let mut core = MicCore {
            mics,
            fir_coeff: [0; 128],
        };
        core.select_fir_coeff(&FIR_DEFAULT).unwrap();
        core
    }

    pub fn set_fir_coeff(&self) -> MicResult<()> {
        self.mics.bus().write(
            memory_map::fpga_address::MICROPHONE_ARRAY,
            as_bytes(&self.fir_coeff),
        );
        Ok(())
    }

    pub fn set_custom_fir_coeff(&mut self, custom_fir: Coeffs) -> MicResult<()> {
        self.fir_coeff = custom_fir;
        self.set_fir_coeff()
    }

    pub fn select_fir_coeff(&mut self, coeffs: &[FirCoeff; NUM_SUPPORTED_FREQ]) -> MicResult<()> {
        let sampling_rate = self.mics.sampling_rate();
        if sampling_rate == 0 {
            return Err(Error::InvalidInput);
        }

        for coeff in coeffs.iter() {
            if coeff.rate == sampling_rate {
                self.fir_coeff = coeff.coeff;
                return self.set_fir_coeff();
            }
        }
        Err(Error::InvalidInput)
    }
}

pub struct FirCoeff {
    rate: u32,
    coeff: Coeffs,
}
