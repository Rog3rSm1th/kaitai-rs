## kaitai-rs

***kaitai-rs*** is a library that allows to parse binary files using [Kaitai Struct](https://kaitai.io/) in pure Rust (it is still a work-in-progress for now). I decided to start this project because all the Kaitai Struct implementations in Rust were either incomplete or no longer maintained.

### Project structure

```
.
├── kaitai-rs               # kaitai-rs library
├── kaitai_struct_formats   # Git submodule for Kaitai Structs samples
├── ksy-parser              # A tool to parse .ksy files and pretty print them
├── LICENSE
└── README.md
```