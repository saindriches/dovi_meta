# **dovi_meta** 
[![CI](https://github.com/saindriches/dovi_meta/workflows/CI/badge.svg)](https://github.com/saindriches/dovi_meta/actions/workflows/ci.yml)
[![Artifacts](https://github.com/saindriches/dovi_meta/workflows/Artifacts/badge.svg)](https://github.com/saindriches/dovi_meta/actions/workflows/release.yml)
[![Github all releases](https://img.shields.io/github/downloads/saindriches/dovi_meta/total.svg)](https://GitHub.com/saindriches/dovi_meta/releases/)


**`dovi_meta`** is a CLI tool for creating Dolby Vision XML metadata from an encoded deliverable with binary metadata.

## **Building**
### **Toolchain**

The minimum Rust version to build **`dovi_meta`** is 1.79.0.

### **Release binary**
To build release binary in `target/release/dovi_meta` run:
```console
cargo build --release
```

## Usage
```properties
dovi_meta [OPTIONS] <SUBCOMMAND>
```
**To get more detailed options for a subcommand**
```properties
dovi_meta <SUBCOMMAND> --help
```

## All options
- `--help`, `--version`
## All subcommands
Currently, the available subcommand is **`convert`** and **`edl`**.

**More information and detailed examples for the subcommands below.**


* ### **convert**
  Convert a binary RPU to XML Metadata (DolbyLabsMDF).
  * Currently, it should support RPU with any Dolby Vision profile using **PQ** as EOTF.
  * Supported XML Version: **CM v2.9** (v2.0.5), **CM v4.0** (v4.0.2 and v5.1.0)
    - The output version is determined by input automatically.
  
  **Arguments**
  * `INPUT`                   Set the input RPU file to use
    - No limitation for RPU file extension.
  * `OUTPUT`                  Set the output XML file location
    - When `OUTPUT` is not set, the output file is `metadata.xml` at current path.
  
  **Options**
  * `-s`, `--size`            Set the canvas size. Use `x` as delimiter
    - Default value is `3840x2160`
  * `-r`, `--rate`            Set the frame rate. Format: integer `NUM` or `NUM/DENOM`
    - Default value is `24000/1001`
  * `-t`, `--skip`            Set the number of frames to be skipped from start
    - Default value is `0`
  * `-n`, `--count`           Set the number of frames to be parsed explicitly

  **Flags**
  * `-6`, `--use-level6`      Use MaxCLL and MaxFALL from RPU, if possible
    - It's not a default behavior, as ST.2086 metadata is not required for a Dolby Vision deliverable.
  * `-d`, `--drop-per-frame`  Drop per-frame metadata in shots
  * `-k`, `--keep-offset`     Keep the offset of frames when `--skip` is set
    
  **Example to get metadata for RPU from a 29.97 fps HD video, dropping first 24 frames**:

  ```console
  dovi_meta convert RPU.bin metadata.xml --skip 24 --rate 30000/1001 --size 1920x1080
  ```
  The default color encoding  is **BT.2020 PQ 16-bit RGB Full Range**.

  The default color space of mastering display and target displays (except the anchor target) is **P3 D65** for CM v2.9 XML, also for CM v4.0 XML when it can't be determined by input.

* ### **edl**
  Convert a binary RPU to EDL (Edit Decision List).
  * Currently, the per-frame metadata in RPU is not parsed to transition.

  **Arguments**
  * `INPUT`                   Set the input RPU file to use
    - No limitation for RPU file extension.
  * `OUTPUT`                  Set the output XML file location
    - When `OUTPUT` is not set, the output file is `metadata.edl` at current path.
  * `CLIP_NAME`               Set the clip name in EDL
    - If there are too many cuts to be saved in a single file,
      multiple files will be saved with a suffix added to the file name.

  **Options**
  * `-r`, `--rate`            Set the frame rate. Format: integer `NUM` or `NUM/DENOM`
    - Default value is `24000/1001`
  * `-s`, `--start-timecode`  Set the starting timecode in timeline. Format: `HH:MM:SS:FF` or integer `FRAMES` offset
    - Default value is `01:00:00:00`
  * `-t`, `--skip`            Set the number of frames to be skipped from start
    - Default value is `0`
  * `-n`, `--count`           Set the number of frames to be parsed explicitly

  **Flags**
  * `-f`, `--force`           Force output even if per-frame RPU is detected

## **Notes**
The current build only support RPU as input. To extract RPU from an HEVC file, see [dovi_tool](https://github.com/quietvoid/dovi_tool) for more info.


Build artifacts can be found in the GitHub Actions.  
More features may or may not be added in the future.
Please report an issue if you have any question.
