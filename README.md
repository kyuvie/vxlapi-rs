# vxlapi-rs(implementing only xlCANdemo)

A wrapper of Vector vxlapi using rust.
Almost of this source code is using vxlapi.h in XL Driver Library as a reference.
Now this software implements minimum functions which can run **xlCANdemo** of XL Driver Library's samples.

**NOTE:**
- I'm not responsible for any accidents and losses which occur by this software.
- Sorry for my terrible English.


## Prerequisites
- Windows 10
- XL Driver Library **20.30.14**

## Usage
1. Read here and download Vector Drvier Setup and XL Driver Library.
https://www.vector.com/jp/ja/products/products-a-z/libraries-drivers/xl-driver-library/#

2. Install them.

3. Clone vxlapi-rs to somewhere and move vxlapi64.dll of XL Driver Library to vxlapi-rs.

4. Run
```
cargo run --bin xlCANdemo -- sample 123
```
