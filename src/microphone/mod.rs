use crate::{
    *,
    bus::{memory_map, Driver, MatrixBus},
};
use heapless::{consts, spsc::Queue, Vec};
use typenum::Unsigned;
use core::mem::MaybeUninit;

pub mod mic_core;

with_std! {
    struct Mic();
    impl Mic {
        pub fn setup_event() -> Result<(), Error> {
            unimplemented!()
        }
        pub fn wait_event() {
            unimplemented!()
        }
    }
}
without_std! {
    mod esp_mic;
    use esp_mic::Mic;
}

type MicChannels = consts::U8;

const MIC_ARRAY_BUFFER_SIZE: usize = 4096;
const MIC_CHANNELS: usize = MicChannels::USIZE;
const NUM_SAMPLES: usize = MIC_ARRAY_BUFFER_SIZE / MIC_CHANNELS;

type SampleData = [i16; NUM_SAMPLES];
type ChannelData = [SampleData; MIC_CHANNELS];
type FifoData = [FifoCircularQueue; MIC_CHANNELS];

type MicResult<T> = Result<T, Error>;

#[derive(Debug, Default, Clone, Copy)]
struct SampleFreq {
    sample_rate: u32,
    constant: u16,
    gain: u16,
}

impl SampleFreq {
    const fn new(sample_rate: u32, constant: u16, gain: u16) -> Self {
        Self {
            sample_rate,
            constant,
            gain,
        }
    }
}
const NUM_SUPPORTED_FREQ: usize = 9;
const SAMPLING_FREQUENCIES: [SampleFreq; NUM_SUPPORTED_FREQ] = [
    SampleFreq::new(8000, 374, 0),
    SampleFreq::new(12000, 249, 2),
    SampleFreq::new(16000, 186, 3),
    SampleFreq::new(22050, 135, 5),
    SampleFreq::new(24000, 124, 5),
    SampleFreq::new(32000, 92, 6),
    SampleFreq::new(44100, 67, 7),
    SampleFreq::new(48000, 61, 7),
    SampleFreq::new(96000, 30, 9),
];

/// (X, Y)
struct MicLocation(f32, f32);
const MIC_LOCATIONS: [MicLocation; MIC_CHANNELS] = [
    MicLocation(20.0908795, -48.5036755),  // M1
    MicLocation(-20.0908795, -48.5036755), // M2
    MicLocation(-48.5036755, -20.0908795), // M3
    MicLocation(-48.5036755, 20.0908795),  // M4
    MicLocation(-20.0908795, 48.5036755),  // M5
    MicLocation(20.0908795, 48.5036755),   // M6
    MicLocation(48.5036755, 20.0908795),   // M7
    MicLocation(48.5036755, -20.0908795),  // M8
];

// These buffers are WAAAAYYY too big for the stack
static mut G_RAW_DATA: ChannelData = [[0i16; NUM_SAMPLES]; MIC_CHANNELS];
static mut G_DELAYED_DATA: ChannelData = [[0i16; NUM_SAMPLES]; MIC_CHANNELS];
static mut G_BEAMFORMED: SampleData = [0i16; NUM_SAMPLES];
static mut G_FIFOS: [MaybeUninit<FifoCircularQueue>; MIC_CHANNELS] = [MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit()];

type FifoCircularQueue = Queue<i16, consts::U32, u8, heapless::spsc::SingleCore>;
pub struct MicArray<'a> {
    bus: &'a dyn MatrixBus,
    gain: u16,
    sample_rate: u32,
    raw_data: &'static mut ChannelData,
    delayed_data: &'static mut ChannelData,
    beamformed: &'static mut SampleData,
    fifos: &'static mut FifoData,
}

impl<'a> MicArray<'a> {
    /// Return an instance of MicArray.
    pub fn new(bus: &'a dyn MatrixBus) -> Self {
        unsafe {
            // TODO: when Queue::u8_sc() is made const we can initialise G_FIFOS with the rest of the globals
            // without wrapping in MaybeUninit
            for fifo in G_FIFOS.iter_mut() {
                *fifo = MaybeUninit::new(FifoCircularQueue::u8_sc());
            }

            let mut mic = MicArray {
                bus,
                gain: 3,
                sample_rate: 16000,
                raw_data: &mut G_RAW_DATA,
                delayed_data: &mut G_DELAYED_DATA,
                beamformed: &mut G_BEAMFORMED,
                fifos: core::mem::transmute::<_, &'static mut FifoData>(&mut G_FIFOS),
            };
            mic.calculate_delays_default(0.0, 0.0);
            Mic::setup_event().unwrap();
            let _ = mic.read_conf_values();
            log::info!("MicArray rate={} gain={}", mic.sampling_rate(), mic.gain());
            mic
        }
    }

    pub fn read(&mut self) -> MicResult<()> {
        Mic::wait_event();

        // Read samples for each channel
        for c in self.raw_data.iter_mut() {
            self.bus.read(
                memory_map::fpga_address::MICROPHONE_ARRAY,
                as_mut_bytes(c),
            );
        }
        for s in 0..NUM_SAMPLES {
            let mut sum = 0i32;
            for c in 0..MIC_CHANNELS {
                // delaying data for beamforming 'delay & sum' algorithm
                self.fifos[c].enqueue(self.raw_data[c][s]).unwrap();
                self.delayed_data[c][s] = self.fifos[c].dequeue().unwrap();

                // accumulation data for beamforming 'delay & sum' algorithm
                sum += self.delayed_data[c][s] as i32;
            }

            // Clamp value to range of i16
            self.beamformed[s] = sum.max(core::i16::MIN as i32).min(core::i16::MAX as i32) as i16;
        }

        Ok(())
    }

    /// Iterator over delay compensated samples for a channel
    pub fn samples_iter(&self, channel: usize) -> impl Iterator<Item = &i16> {
        self.delayed_data[channel].iter()
    }

    pub fn sampling_rate(&self) -> u32 {
        self.sample_rate
    }
    pub fn gain(&self) -> u16 {
        self.gain
    }
    pub const fn channels(&self) -> u16 {
        MIC_CHANNELS as u16
    }
    pub const fn number_of_samples(&self) -> u16 {
        NUM_SAMPLES as u16
    }

    pub fn read_gain(&mut self) -> MicResult<()> {
        let mut buffer = [0u16; 1];
        self.bus.read(
            memory_map::fpga_address::CONF + 0x07,
            as_mut_bytes(&mut buffer),
        );
        self.gain = buffer[0];
        Ok(())
    }

    pub fn write_gain(&mut self, gain: u16) -> MicResult<()> {
        let buffer = [gain];
        self.bus
            .write(memory_map::fpga_address::CONF + 0x07, as_bytes(&buffer));
        self.gain = gain;
        Ok(())
    }

    pub fn read_sample_rate(&mut self) -> MicResult<()> {
        let mut buffer = [0u16; 1];
        self.bus.read(
            memory_map::fpga_address::CONF + 0x06,
            as_mut_bytes(&mut buffer),
        );
        let sample = SAMPLING_FREQUENCIES
            .iter()
            .find(|s| s.constant == buffer[0]);
        if let Some(SampleFreq { sample_rate, .. }) = sample {
            self.sample_rate = *sample_rate;
            Ok(())
        } else {
            Err(Error::InvalidInput)
        }
    }

    pub fn write_sample_rate(&mut self, sample_rate: u32) -> MicResult<()> {
        let SampleFreq { gain, constant, .. } = SAMPLING_FREQUENCIES
            .iter()
            .find(|s| s.sample_rate == sample_rate)
            .ok_or(Error::InvalidInput)?;

        self.write_gain(*gain)?;
        let buffer = [constant];
        self.bus
            .write(memory_map::fpga_address::CONF + 0x06, as_bytes(&buffer));
        self.sample_rate = sample_rate;

        Ok(())
    }

    pub fn read_conf_values(&mut self) -> MicResult<()> {
        self.read_gain()?;
        self.read_sample_rate()
    }

    pub fn calculate_delays_default(&mut self, azimutal_angle: f32, polar_angle: f32) {
        self.calculate_delays(azimutal_angle, polar_angle, 100.0, 320_000.0);
    }

    pub fn calculate_delays(
        &mut self,
        azimutal_angle: f32,
        polar_angle: f32,
        radial_distance_mm: f32,
        sound_speed_mmseg: f32,
    ) {
        let x = radial_distance_mm * libm::sinf(azimutal_angle) * libm::cosf(polar_angle);
        let y = radial_distance_mm * libm::sinf(azimutal_angle) * libm::sinf(polar_angle);
        let z = radial_distance_mm * libm::cosf(azimutal_angle);

        // distances from source position to each microphone
        let mut distance_map: Vec<(f32, usize), MicChannels> = Default::default();
        // distance_map.resize_default(MIC_CHANNELS);
        for c in 0..MIC_CHANNELS {
            let distance = libm::sqrtf(
                libm::powf(MIC_LOCATIONS[c].0 - x, 2.0)
                    + libm::powf(MIC_LOCATIONS[c].1 - y, 2.0)
                    + libm::powf(z, 2.0),
            );
            distance_map.push((distance, c)).unwrap();
        }

        // fifo resize for delay compensation
        let min_distance = distance_map
            .iter()
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .unwrap()
            .0;
        for (distance, channel) in distance_map {
            let delay = libm::roundf(
                (distance - min_distance) * self.sample_rate as f32 / sound_speed_mmseg,
            ) as usize;
            for _ in 0..delay {
                self.fifos[channel].enqueue(0).unwrap();
            }
        }
    }
}

impl Driver for MicArray<'_> {
    fn bus(&self) -> &dyn MatrixBus {
        self.bus
    }
}
