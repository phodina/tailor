# tailor
A Rust tool to generate MCU register definitions based on SVD file.

## Usage
Run this command:
```
tailor --output OUTPUT --svd SVD --templates TEMPLATES
```
* OUTPUT - directory with the rendered files containing register definitions
* SVD - file containing description of the MCU
* TEMPLATES - template files for peripherals