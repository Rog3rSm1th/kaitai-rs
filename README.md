## kaitai-rs

**kaitai-rs** is a library that allows to parse binary files using [Kaitai Struct](https://kaitai.io/) in pure Rust (it is still a work-in-progress for now). I decided to start this project because all the Kaitai Struct implementations in Rust were either incomplete or no longer maintained.

> [!IMPORTANT]
> Kaitai-rs is currently in development, and not all features of the `.ksy` format description language are fully supported yet. New features are being added gradually as the project progresses.

### Project structure

```
.
├── kaitai-rs               # kaitai-rs library
├── kaitai_struct_formats   # Git submodule for Kaitai Structs samples
├── kaitai-parser           # A tool to parse files using a .ksy format description and print their AST
├── LICENSE
└── README.md
```