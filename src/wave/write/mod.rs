mod writer;

use super::*;
use color_eyre::eyre::{Context as _, OptionExt as _};
use std::io::{Result, Write};
use writer::CountingWriter;

fn size_wav_file(wav: &WaveFile) -> color_eyre::eyre::Result<(u32, u32, u32)> {
    let riff_size: u32 = 4u32;
    let header_size: u32 = 8u32;
    let fmt_size: u32 = header_size + 16u32;
    let mut chunk_size = riff_size + fmt_size + header_size;

    let cue_size: u32 = match &wav.cue {
        Some(cue) => {
            let len: u32 = cue.len().try_into().wrap_err("Way too many cue points")?;
            let cue_size = len
                .checked_mul(24)
                .ok_or_eyre("Way too many cue points")?
                .checked_add(4u32)
                .ok_or_eyre("Way too many cue points")?;
            // ok, because chunk_size is fixed and small here
            chunk_size += header_size;
            chunk_size = chunk_size
                .checked_add(cue_size)
                .ok_or_eyre("Way too many cue points")?;
            cue_size
        }
        None => 0,
    };

    let data_size: u32 = wav
        .data
        .len()
        .try_into()
        .wrap_err("DATA chunk is greater than 4 GiB")?;
    let data_pad = data_size & 1;

    let chunk_size = chunk_size
        .checked_add(data_size)
        .ok_or_eyre("File size is greater than 4 GiB")?
        .checked_add(data_pad)
        .ok_or_eyre("File size is greater than 4 GiB")?;

    Ok((chunk_size, cue_size, data_size))
}

fn write_chunk_header(
    write: &mut CountingWriter<impl Write>,
    chunk_id: ChunkId,
    size: u32,
) -> Result<()> {
    tracing::trace!(
        "writing chunk `{}`, size {} at {}",
        chunk_id,
        size,
        write.offset
    );
    write.write_all(chunk_id.as_ref())?;
    write.write_u32(size)?;
    Ok(())
}

fn write_riff_chunk(write: &mut CountingWriter<impl Write>, size: u32) -> Result<()> {
    write_chunk_header(write, RIFF_CHUNK_ID, size)?;
    write.write_all(FORM_TYPE_WAVE.as_ref())?;
    Ok(())
}

fn write_fmt_chunk(write: &mut CountingWriter<impl Write>, fmt: &Format) -> Result<()> {
    write_chunk_header(write, FMT_CHUNK_ID, 16)?;

    let block_align = fmt.bits_per_sample.block_align(fmt.channels);
    let samples_per_sec = fmt.samples_per_sec.as_u32();
    let avg_bytes_per_sec = samples_per_sec * block_align as u32;

    write.write_u16(WAVE_FORMAT_PCM)?;
    write.write_u16(fmt.channels.as_u16())?;
    write.write_u32(samples_per_sec)?;
    write.write_u32(avg_bytes_per_sec)?;
    write.write_u16(block_align)?;
    write.write_u16(fmt.bits_per_sample.as_u16())?;
    Ok(())
}

fn write_cue_point(
    write: &mut CountingWriter<impl Write>,
    index: u32,
    position: u32,
) -> Result<()> {
    tracing::trace!("writing cue point {} at {}", index, write.offset);
    write.write_u32(index)?;
    write.write_u32(position)?;
    write.write_all(DATA_CHUNK_ID.as_ref())?;
    write.write_u32(0)?; // chunk_start
    write.write_u32(0)?; // block_start
    write.write_u32(position)?; // sample_start
    tracing::trace!("cue point {} is {}", index, position);
    Ok(())
}

fn write_cue_chunk(write: &mut CountingWriter<impl Write>, cue: &[u32], size: u32) -> Result<()> {
    write_chunk_header(write, CUE_CHUNK_ID, size)?;

    // Cast safety: this has already been validated by `size_wav_file`
    let cue_point_count = cue.len() as u32;
    write.write_u32(cue_point_count)?;

    for (position, index) in cue.iter().copied().zip(1u32..) {
        write_cue_point(write, index, position)?
    }
    Ok(())
}

fn write_data_chunk(write: &mut CountingWriter<impl Write>, data: &[u8], size: u32) -> Result<()> {
    write_chunk_header(write, DATA_CHUNK_ID, size)?;
    write.write_all(data)?;
    if (size & 1) == 1 {
        write.write_u8(0)?;
    }
    Ok(())
}

fn write_wav_file(
    write: &mut CountingWriter<impl Write>,
    wav: &WaveFile,
) -> color_eyre::eyre::Result<()> {
    let (total_size, cue_size, data_size) = size_wav_file(wav)?;

    write_riff_chunk(write, total_size)?;
    write_fmt_chunk(write, &wav.fmt)?;

    if let Some(cue) = &wav.cue {
        write_cue_chunk(write, cue, cue_size)?;
    }

    write_data_chunk(write, &wav.data, data_size)?;
    tracing::trace!("wrote {} bytes", write.offset);
    Ok(())
}

pub(crate) fn write(wav: &WaveFile) -> color_eyre::eyre::Result<Vec<u8>> {
    let mut write = CountingWriter::new(Vec::new());
    write_wav_file(&mut write, wav)?;
    Ok(write.into_inner())
}
