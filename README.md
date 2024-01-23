# WAV Header Information Command Line Tool (Similar to ffprobe)

This tool is designed for detailed analysis and display of WAV file header information, akin to ffprobe but specifically focused on the WAV format. For in-depth details about the WAV file format, please refer to the following link:

* WAV File Format Documentation(https://docs.fileformat.com/audio/wav/)

## wav format information

* https://wavefilegem.com/how_wave_files_work.html

* Integer PCM at 8, 16, 24, or 32 bits per sample (format tag 1)
* Floating point PCM at 32 or 64 bits per sample (format tag 3)
* The formats above when using WAVE_FORMAT_EXTENSIBLE (format tag 65534)

## Download sample file

* https://file-examples.com/storage/fe793dd9be65a9b389251ea/2017/11/file_example_WAV_1MG.wav

* https://mauvecloud.net/sounds/index.html

## Usage

```bash
$ cargo build --release
$ cargo install --path .
```

```bash
$ myffprobe -i filename.wav
RIFF:                  RIFF
Chunk size:            1767588
Format:                WAVE
fmt identifier:        fmt 
fmt chunk size:        40
sound format:          Extensible
channels:              2
sample rate:           44100
byte rate:             352800
block size:            8
bits per sample:       32
extend parameter size: Some(22)
extend parameter:      Some([32, 0, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 16, 0, 128, 0, 0, 170, 0, 56, 155, 113])
subchunk identifier:   data
subchunk size:         1767528
```

## Log

* Modified the Sound identifier to accept inputs other than 'data'.