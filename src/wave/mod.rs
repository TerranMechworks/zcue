mod chunk_id;
mod parse;
mod read;
mod write;

pub(crate) use chunk_id::ChunkId;
pub(crate) use parse::{cue_from_wav, cue_to_wav};
pub(crate) use read::read;
use serde::{Deserialize, Serialize};
use std::fmt;
pub(crate) use write::write;

const RIFF_CHUNK_ID: ChunkId = ChunkId::new(*b"RIFF");
const FMT_CHUNK_ID: ChunkId = ChunkId::new(*b"fmt ");
const CUE_CHUNK_ID: ChunkId = ChunkId::new(*b"cue ");
const DATA_CHUNK_ID: ChunkId = ChunkId::new(*b"data");
const FORM_TYPE_WAVE: ChunkId = ChunkId::new(*b"WAVE");
const WAVE_FORMAT_PCM: u16 = 1;

pub(crate) struct WaveFile {
    pub(crate) fmt: Format,
    pub(crate) cue: Option<Vec<u32>>,
    pub(crate) data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Channels {
    One,
}

impl Channels {
    pub(crate) fn from_u16(value: u16) -> Option<Self> {
        match value {
            1 => Some(Self::One),
            _ => None,
        }
    }

    pub(crate) fn as_u16(&self) -> u16 {
        match self {
            Self::One => 1,
        }
    }
}

impl fmt::Display for Channels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One => f.write_str("1"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum BitsPerSample {
    Eight,
    Sixteen,
}

impl BitsPerSample {
    pub(crate) fn from_u16(value: u16) -> Option<Self> {
        match value {
            8 => Some(Self::Eight),
            16 => Some(Self::Sixteen),
            _ => None,
        }
    }

    pub(crate) fn as_u16(&self) -> u16 {
        match self {
            Self::Eight => 8,
            Self::Sixteen => 16,
        }
    }

    pub(crate) fn block_align(&self, channels: Channels) -> u16 {
        // for two channels, the values would be doubled
        match channels {
            Channels::One => match self {
                Self::Eight => 1,
                Self::Sixteen => 2,
            },
        }
    }
}

impl fmt::Display for BitsPerSample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eight => f.write_str("8"),
            Self::Sixteen => f.write_str("16"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum SamplesPerSec {
    Hz11025,
    Hz22000,
    Hz22040,
    Hz22050,
    Hz44100,
}

impl SamplesPerSec {
    const EXPECTED: &'static str = "11025, 22000, 22040, 22050, or 44100";

    pub(crate) fn from_u32(value: u32) -> Option<Self> {
        match value {
            11025 => Some(Self::Hz11025),
            22000 => Some(Self::Hz22000),
            22040 => Some(Self::Hz22040),
            22050 => Some(Self::Hz22050),
            44100 => Some(Self::Hz44100),
            _ => None,
        }
    }

    pub(crate) fn as_u32(&self) -> u32 {
        match self {
            Self::Hz11025 => 11025,
            Self::Hz22000 => 22000,
            Self::Hz22040 => 22040,
            Self::Hz22050 => 22050,
            Self::Hz44100 => 44100,
        }
    }
}

impl fmt::Display for SamplesPerSec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hz11025 => f.write_str("11025"),
            Self::Hz22000 => f.write_str("22000"),
            Self::Hz22040 => f.write_str("22040"),
            Self::Hz22050 => f.write_str("22050"),
            Self::Hz44100 => f.write_str("44100"),
        }
    }
}

impl Serialize for SamplesPerSec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.as_u32())
    }
}

impl<'de> Deserialize<'de> for SamplesPerSec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error as _;
        let value = u32::deserialize(deserializer)?;
        Self::from_u32(value).ok_or_else(|| {
            D::Error::invalid_value(serde::de::Unexpected::Unsigned(value as _), &Self::EXPECTED)
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Format {
    pub(crate) channels: Channels,
    pub(crate) samples_per_sec: SamplesPerSec,
    pub(crate) bits_per_sample: BitsPerSample,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct SamplePoints {
    pub(crate) samples_per_sec: SamplesPerSec,
    pub(crate) sample_starts: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct CuePoints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) sample_points: Option<SamplePoints>,
    pub(crate) timestamps: Vec<f32>,
}
