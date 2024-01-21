mod reader;

use super::*;
use color_eyre::eyre::{bail, eyre, Context, Result};
use reader::CountingReader;
use std::io::{self, Cursor, Read};

fn read_chunk_header(read: &mut CountingReader<impl Read>) -> io::Result<(ChunkId, u32)> {
    let chunk_id = read.read_chunk_id()?;
    let chunk_size = read.read_u32()?;

    tracing::trace!(
        "reading chunk `{}`, size {} at {}",
        chunk_id,
        chunk_size,
        read.offset - 8,
    );

    Ok((chunk_id, chunk_size))
}

fn read_riff_chunk(read: &mut CountingReader<impl Read>, total_size: u32) -> Result<()> {
    let (chunk_id, chunk_size) = read_chunk_header(read)?;
    if chunk_id != RIFF_CHUNK_ID {
        bail!(
            "Expected `RIFF chunk ID` == {:#?}, but was {:#?} (at {})",
            RIFF_CHUNK_ID,
            chunk_id,
            read.prev - 4,
        );
    }

    let expected_size = total_size.saturating_sub(8);
    tracing::trace!(
        "file size: {}, chunk size: {}, expected: {}",
        total_size,
        chunk_size,
        expected_size,
    );
    if chunk_size != expected_size {
        bail!(
            "Expected `RIFF chunk size` == {:#?}, but was {:#?} (at {})",
            chunk_size,
            expected_size,
            read.prev,
        );
    }

    let form_type = read.read_chunk_id()?;
    if form_type != FORM_TYPE_WAVE {
        bail!(
            "Expected `RIFF form type` == {:#?}, but was {:#?} (at {})",
            FORM_TYPE_WAVE,
            form_type,
            read.prev,
        );
    }

    Ok(())
}

fn read_fmt_chunk(read: &mut CountingReader<impl Read>) -> Result<Format> {
    let (chunk_id, chunk_size) = read_chunk_header(read)?;
    if chunk_id != FMT_CHUNK_ID {
        bail!(
            "Expected `FMT chunk ID` == {:#?}, but was {:#?} (at {})",
            FMT_CHUNK_ID,
            chunk_id,
            read.prev - 4,
        );
    }

    let chunk_size_pos = read.prev;

    let format_tag = read.read_u16()?;
    if format_tag != WAVE_FORMAT_PCM {
        bail!(
            "Expected `FMT format tag` == {:#?}, but was {:#?} (at {})",
            WAVE_FORMAT_PCM,
            format_tag,
            read.prev,
        );
    }

    // this is only valid for PCM files
    // 18 is an invalid WAVEFORMATEX without size
    if !(chunk_size == 16 || chunk_size == 18) {
        bail!(
            "Expected `FMT chunk size` == {:#?} or {:#?}, but was {:#?} (at {})",
            16,
            18,
            chunk_size,
            chunk_size_pos,
        );
    }

    let channels = read.read_u16()?;
    let channels = Channels::from_u16(channels).ok_or_else(|| {
        eyre!(
            "Expected `FMT channels` == {:#?}, but was {:#?} (at {})",
            1,
            channels,
            read.prev,
        )
    })?;

    let samples_per_sec = read.read_u32()?;
    let samples_per_sec = SamplesPerSec::from_u32(samples_per_sec).ok_or_else(|| {
        eyre!(
            "Unknown samples per second {} (at {})",
            samples_per_sec,
            read.prev,
        )
    })?;

    // PCM: channels * bitsPerSecond * (bitsPerSample / 8)
    let avg_bytes_per_sec = read.read_u32()?;
    let avg_bytes_per_sec_pos = read.prev;

    // PCM: channels * (bitsPerSample / 8)
    let block_align = read.read_u16()?;
    let block_align_pos = read.prev;

    let bits_per_sample = read.read_u16()?;
    let bits_per_sample = BitsPerSample::from_u16(bits_per_sample).ok_or_else(|| {
        eyre!(
            "Expected `FMT bits per sample` == {:#?} or {:#?}, but was {:#?} (at {})",
            8,
            16,
            bits_per_sample,
            read.prev,
        )
    })?;

    let expected_block_align = bits_per_sample.block_align(channels);
    if block_align != expected_block_align {
        bail!(
            "Expected `FMT block align` == {:#?}, but was {:#?} (at {})",
            expected_block_align,
            block_align,
            block_align_pos,
        );
    }

    // this might not work for 2 channels? but should
    let expected_avg_bytes_per_sec = samples_per_sec.as_u32() * block_align as u32;
    if avg_bytes_per_sec != expected_avg_bytes_per_sec {
        bail!(
            "Expected `FMT avg bytes per sec` == {:#?}, but was {:#?} (at {})",
            expected_avg_bytes_per_sec,
            avg_bytes_per_sec,
            avg_bytes_per_sec_pos,
        );
    }

    // 18 is an invalid WAVEFORMATEX without size
    if chunk_size == 18 {
        let extension_size = read.read_u16()?;
        if extension_size != 0 {
            bail!(
                "Expected `FMT extension size` == {:#?}, but was {:#?} (at {})",
                0,
                extension_size,
                read.prev,
            );
        }
    }

    Ok(Format {
        channels,
        samples_per_sec,
        bits_per_sample,
    })
}

fn read_cue_point(read: &mut CountingReader<impl Read>, index: u32) -> Result<u32> {
    tracing::trace!("reading cue point {} at {}", index, read.offset);

    let id = read.read_u32()?;
    if id != index {
        bail!(
            "Expected `cue point id` == {:#?}, but was {:#?} (at {})",
            index,
            id,
            read.prev,
        );
    }

    let position = read.read_u32()?;

    let data_chunk_id = read.read_chunk_id()?;
    if data_chunk_id != DATA_CHUNK_ID {
        bail!(
            "Expected `cue point data chunk id` == {:#?}, but was {:#?} (at {})",
            DATA_CHUNK_ID,
            data_chunk_id,
            read.prev,
        );
    }

    let chunk_start = read.read_u32()?;
    if chunk_start != 0 {
        bail!(
            "Expected `cue point chunk start` == {:#?}, but was {:#?} (at {})",
            0,
            chunk_start,
            read.prev,
        );
    }

    let block_start = read.read_u32()?;
    if block_start != 0 {
        bail!(
            "Expected `cue point block start` == {:#?}, but was {:#?} (at {})",
            0,
            block_start,
            read.prev,
        );
    }

    let sample_start = read.read_u32()?;
    if sample_start != position {
        bail!(
            "Expected `cue point sample start` == {:#?}, but was {:#?} (at {})",
            position,
            sample_start,
            read.prev,
        );
    }

    tracing::trace!("cue point {} is {}", index, position);
    Ok(position)
}

const CUE_CHUNK_MIN_SIZE: u32 = 4 + 24;

fn read_cue_chunk(read: &mut CountingReader<impl Read>, chunk_size: u32) -> Result<Vec<u32>> {
    let chunk_size_pos = read.prev;

    if chunk_size < CUE_CHUNK_MIN_SIZE {
        bail!(
            "Expected `CUE chunk size` >= {}, but was {} (at {})",
            CUE_CHUNK_MIN_SIZE,
            chunk_size,
            chunk_size_pos,
        );
    }

    let cue_point_count = read.read_u32()?;
    let expected_size = 4 + cue_point_count * 24;

    if chunk_size != expected_size {
        bail!(
            "Expected `CUE chunk size` == {}, but was {} (at {})",
            chunk_size,
            expected_size,
            chunk_size_pos,
        );
    }

    (1..=cue_point_count)
        .map(|index| read_cue_point(read, index))
        .collect()
}

fn read_data_chunk(read: &mut CountingReader<impl Read>, chunk_size: u32) -> Result<Vec<u8>> {
    // Cast safety: usize >= u32
    let mut buf = vec![0u8; chunk_size as usize];
    read.read_exact(&mut buf)?;

    if (chunk_size & 1) == 1 {
        let pad = read.read_u8()?;
        if pad != 0 {
            bail!(
                "Expected `DATA padding` == {}, but was {} (at {})",
                0,
                pad,
                read.prev,
            );
        }
    }

    Ok(buf)
}

fn read_wav_file(read: &mut CountingReader<impl Read>, total_size: u32) -> Result<WaveFile> {
    // the RIFF chunk must be first
    read_riff_chunk(read, total_size)?;

    // the FMT chunk must be second
    let fmt = read_fmt_chunk(read)?;

    let mut cue = None;
    let mut data = None;
    loop {
        let chunk_start_pos = read.offset;
        let (chunk_id, chunk_size) = read_chunk_header(read)?;
        match chunk_id {
            FMT_CHUNK_ID => {
                bail!("Duplicate FMT chunk (at {})", chunk_start_pos);
            }
            CUE_CHUNK_ID => {
                if cue.is_some() {
                    bail!("Duplicate CUE chunk (at {})", chunk_start_pos);
                }
                cue = Some(read_cue_chunk(read, chunk_size)?);
            }
            DATA_CHUNK_ID => {
                if data.is_some() {
                    bail!("Duplicate DATA chunk (at {})", chunk_start_pos);
                }
                data = Some(read_data_chunk(read, chunk_size)?);
                break;
            }
            _ => {
                bail!("Unknown chunk `{}` (at {})", chunk_id, chunk_start_pos);
            }
        }
    }

    let data = data.ok_or_else(|| eyre!("WAVE file contains no DATA chunk"))?;

    if read.offset != total_size {
        bail!(
            "Expected `bytes read` == {}, but was {} (at {})",
            total_size,
            read.offset,
            read.offset,
        )
    }

    tracing::trace!("read {} bytes", read.offset);
    Ok(WaveFile { fmt, cue, data })
}

pub(crate) fn read(buf: &[u8]) -> Result<WaveFile> {
    let total_size = buf
        .len()
        .try_into()
        .wrap_err("File size is greater than 4 GiB")?;
    let cursor = Cursor::new(buf);
    let mut read = CountingReader::new(cursor);
    read_wav_file(&mut read, total_size)
}
