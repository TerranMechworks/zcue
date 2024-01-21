use super::{BitsPerSample, CuePoints, SamplePoints, WaveFile};
use color_eyre::eyre::{eyre, OptionExt as _, Result};
use std::num::FpCategory;

pub(crate) fn cue_from_wav(wav: &WaveFile) -> Result<CuePoints> {
    let samples_per_sec = wav.fmt.samples_per_sec;
    let sample_starts = wav.cue.clone().ok_or_eyre("File contains no cue points")?;

    // "validate" sample starts
    let data_chunk_size = wav.data.len() as u32;
    let bytes_per_sample = match wav.fmt.bits_per_sample {
        BitsPerSample::Eight => 1u32,
        BitsPerSample::Sixteen => 2u32,
    };
    let sample_count = data_chunk_size / bytes_per_sample;
    for (sample_start, index) in sample_starts.iter().copied().zip(1..) {
        if sample_start > sample_count {
            tracing::warn!(
                "Invalid cue point {}: sample start {} > sample count {}",
                index,
                sample_start,
                sample_count
            );
        }
    }

    // convert sample starts to timestamps
    let hz = samples_per_sec.as_u32() as f32;
    let timestamps = sample_starts
        .iter()
        .copied()
        .map(|s| (s as f32) / hz)
        .collect();

    let sample_points = Some(SamplePoints {
        samples_per_sec,
        sample_starts,
    });

    Ok(CuePoints {
        sample_points,
        timestamps,
    })
}

pub(crate) fn cue_to_wav(wav: &mut WaveFile, cue: CuePoints) -> Result<()> {
    let samples_per_sec = wav.fmt.samples_per_sec;
    if wav.cue.is_some() {
        tracing::warn!("Input file contains cue points, overwriting...");
    }

    // convert timestamps to sample starts
    let hz = samples_per_sec.as_u32() as f32;
    let sample_starts = cue
        .timestamps
        .iter()
        .copied()
        .map(|ts| match ts.classify() {
            FpCategory::Infinite => Err(eyre!("Timestamp is invalid (inf)")),
            FpCategory::Nan => Err(eyre!("Timestamp is invalid (nan)")),
            FpCategory::Zero => Ok(0),
            FpCategory::Subnormal | FpCategory::Normal => {
                let ss = (ts * hz).round();
                if ss < 0.0 {
                    Err(eyre!("Timestamp is invalid (neg)"))
                } else if ss > (u32::MAX as f32) {
                    Err(eyre!("Timestamp is invalid (max)"))
                } else {
                    Ok(ss as u32)
                }
            }
        })
        .collect::<Result<Vec<u32>>>()?;

    wav.cue = Some(sample_starts);
    Ok(())
}
