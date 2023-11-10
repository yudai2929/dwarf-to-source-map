# dwarf-to-source-map

`dwarf-to-source-map` is a Rust-based tool designed to generate source maps from DWARF debug information. This tool is particularly useful for developers working with compiled code, such as WebAssembly, as it allows them to map the compiled code back to the original source for easier debugging and analysis.

## Installation

To install `dwarf-to-source-map`, you need to have Rust and Cargo installed on your system. Follow these steps:

1. Clone the repository:
```
git clone https://github.com/yudai2929/dwarf-to-source-map.git
```
2. Navigate to the project directory:
```
cd dwarf-to-source-map
```
3. Build the project using Cargo:
```
cargo build
```


## Usage

After installing, you can run `dwarf-to-source-map` using Cargo. The basic command structure is as follows:

```
target/debug/map-rust -- [OPTIONS]
```
Replace `[OPTIONS]` with the appropriate options and arguments for your specific use case. For more detailed information on available options and their usage, refer to the project's documentation or use the `--help` flag.

