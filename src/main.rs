use core::panic;
use std::fmt::write;
use std::io::{Seek, SeekFrom, Read};
use std::fs::{File};
use std::path::{Path};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

#[derive(Debug)]
enum FormatTag {
    INTERGERPCM = 1,
    ADPCM = 2,
    FLOATPCM = 3,
    ALAW = 6,
    MULAW = 7,
    EXTENSIBLE = 65534
}

impl std::fmt::Display for FormatTag {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = match self {
            FormatTag::INTERGERPCM => "Integer PCM",
            FormatTag::ADPCM => "ADPCM",
            FormatTag::FLOATPCM => "Float PCM",
            FormatTag::ALAW => "A-Law",
            FormatTag::MULAW => "Mu-Law",
            FormatTag::EXTENSIBLE => "Extensible",
        };        
        write!(f, "{}", args);        
        Ok(())
    }
}


impl Into<u16> for FormatTag{
    fn into(self) -> u16 {
        match self {
            FormatTag::INTERGERPCM => 1,
            FormatTag::ADPCM => 2,
            FormatTag::FLOATPCM => 3,
            FormatTag::ALAW => 6,
            FormatTag::MULAW => 7,
            FormatTag::EXTENSIBLE => 65534,
        }
    }
}

impl From<u16> for FormatTag{
    fn from(value: u16) -> Self {
        match value {
            1 => FormatTag::INTERGERPCM,
            2 => FormatTag::ADPCM,
            3 => FormatTag::FLOATPCM,
            6 => FormatTag::ALAW,
            7 => FormatTag::MULAW,
            65534 => FormatTag::EXTENSIBLE,
            _ => panic!("Unknown format tag: {}", value),
        }
    }
}

#[allow(unused_must_use)]
#[derive(Debug)]
struct Wavinfo {
    riff: String,
    chunk_size: u32,
    format: String,
    fmt_identifier: String,
    fmt_chunk_size: u32,
    sound_format: FormatTag,
    channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_size: u16,
    bits_per_sample: u16,
    extend_parameter_size: Option<u16>,
    extend_parameter: Option<Vec<u8>>,
    subchunk_identifier: String,
    subchunk_size: u32,    
    data: Vec<u8>, 
}

impl std::fmt::Display for Wavinfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "RIFF:                  {}\n", self.riff)?;
        write!(f, "Chunk size:            {}\n", self.chunk_size)?;
        write!(f, "Format:                {}\n", self.format)?;
        write!(f, "fmt identifier:        {}\n", self.fmt_identifier)?;
        write!(f, "fmt chunk size:        {}\n", self.fmt_chunk_size)?;
        write!(f, "sound format:          {}\n", self.sound_format)?;
        write!(f, "channels:              {}\n", self.channels)?;
        write!(f, "sample rate:           {}\n", self.sample_rate)?;
        write!(f, "byte rate:             {}\n", self.byte_rate)?;
        write!(f, "block size:            {}\n", self.block_size)?;
        write!(f, "bits per sample:       {}\n", self.bits_per_sample)?;
        write!(f, "extend parameter size: {:?}\n", self.extend_parameter_size)?;
        write!(f, "extend parameter:      {:?}\n", self.extend_parameter)?;
        write!(f, "subchunk identifier:   {}\n", self.subchunk_identifier)?;
        write!(f, "subchunk size:         {}\n", self.subchunk_size)?;
        //write!(f, "data: {:?}\n", self.data)?;
        Ok(())
    }
}


impl Wavinfo {
    fn read(path: String) -> Self {
        let path = Path::new(&path);
        if !path.exists() {
            panic!("File not found : {:?}", path);
        }
        let mut file = File::open(path).expect("Failed to open file");
        file.seek(SeekFrom::Start(0)).expect("Failed to seek");
        let mut buffer = [0; 4];
        file.read(&mut buffer).expect("Failed to read buffer");
        let riff = String::from_utf8_lossy(&buffer);
        if riff != "RIFF" {
            panic!("Not a RIFF file, first 4 bytes contains {:?}", &riff);
        }
        let mut buffer = [0; 4];        
        file.read(&mut buffer).expect("Failed to read buffer: chunk size");
        let chunk_size = u32::from_le_bytes(buffer);

        let mut buffer = [0; 4];
        file.read(&mut buffer).expect("Failed to read buffer: format");
        let format = String::from_utf8_lossy(&buffer);
        if format != "WAVE" {
            panic!("Not a WAVE file, format is {:?}", &format);
        }

        let mut buffer = [0; 4];
        file.read(&mut buffer).expect("Failed to read buffer: fmt identifier");
        let fmt_identifier = String::from_utf8_lossy(&buffer);
        if fmt_identifier != "fmt " {
            panic!("Not a fmt chunk, identifier is {:?}", &fmt_identifier);
        }

        let mut buffer = [0; 4];
        file.read(&mut buffer).expect("Failed to read buffer: fmt chunk size");
        let fmt_chunk_size = u32::from_le_bytes(buffer);

        let mut buffer = [0; 2];
        file.read(&mut buffer).expect("Failed to read buffer: sound format");
        let sound_format = u16::from_le_bytes(buffer);

        let mut buffer = [0; 2];
        file.read(&mut buffer).expect("Failed to read buffer: channels");
        let channels = u16::from_le_bytes(buffer);

        let mut buffer = [0; 4];
        file.read(&mut buffer).expect("Failed to read buffer: sample rate");
        let sample_rate = u32::from_le_bytes(buffer);

        let mut buffer = [0; 4];
        file.read(&mut buffer).expect("Failed to read buffer: byte rate");
        let byte_rate = u32::from_le_bytes(buffer);

        let mut buffer = [0; 2];
        file.read(&mut buffer).expect("Failed to read buffer: block size");
        let block_size = u16::from_le_bytes(buffer);

        let mut buffer = [0; 2];
        file.read(&mut buffer).expect("Failed to read buffer: bits per sample");
        let bits_per_sample = u16::from_le_bytes(buffer);

        let extend_parameter_size = if sound_format != FormatTag::INTERGERPCM.into() {
            let mut buffer = [0; 2];
            file.read(&mut buffer).expect("Failed to read buffer: extend parameter size");
            Some(u16::from_le_bytes(buffer))
        } else {
            None
        };
        let extend_parameter = match extend_parameter_size {
            Some(size) => {
                let mut buffer = vec![0; size as usize];
                file.read(&mut buffer).expect("Failed to read buffer: extend parameter");
                Some(buffer)
            },
            None => None,
        };

        let mut buffer = [0; 4];
        file.read(&mut buffer).expect("Failed to read buffer: subchunk identifier");
        let subchunk_identifier = String::from_utf8_lossy(&buffer);
        if subchunk_identifier != "data" {
            panic!("Not a data chunk, identifier is {:?}", &subchunk_identifier);
        }

        let mut buffer = [0; 4];
        file.read(&mut buffer).expect("Failed to read buffer: subchunk size");
        let subchunk_size = u32::from_le_bytes(buffer);

        let mut buffer = vec![0; subchunk_size as usize];
        file.read(&mut buffer).expect("Failed to read buffer: data");

        Wavinfo {
            riff: String::from("RIFF"),
            chunk_size,
            format: String::from("WAVE"),
            fmt_identifier: String::from("fmt "),
            fmt_chunk_size,
            sound_format: FormatTag::from(sound_format),
            channels,
            sample_rate,
            byte_rate,
            block_size,
            bits_per_sample,
            extend_parameter_size,
            extend_parameter,
            subchunk_identifier: String::from("data"),
            subchunk_size,
            data: buffer,
        }
    }
}

fn main() {
    let args = Args::parse();
    let wavinfo = Wavinfo::read(args.input);
    println!("{}", wavinfo);
}
