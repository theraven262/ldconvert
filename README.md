# ldconvert

## LUT-Diffuse Converter

Creates an error-diffused LUT from an input palette. Can also quantize images using the resulting LUT.

The approach uses "minimized average error" error-diffusion algorithm on the LUT, performed over the OKLAB colorspace. The algorithm has been modified so as to diffuse over all three channels. Color distance is calculated in the OKLAB colorspace.

There are two LUTs generated: the internal one which contains colors in the OKLAB colorspace and the external one which contains colors in the sRGB space. The resolution of each LUT can be configured. Quantization and error diffusion are done over the OKLAB LUT.

Source palette is a png file, colors can repeat, but increase calculation time.
Alpha channel is unaffected by the conversion. 
Palette index 0 is skipped.

**Command Overview:**

* [`ldconvert`↴](#ldconvert)

## `ldconvert`

LUT-Diffuse Converter. Makes a diffused lookup table from a color palette. Converts images into the desired color palette.

**Usage:** `ldconvert [OPTIONS]`

###### **Options:**

* `-p`, `--palette-path <PALETTE_PATH>` — Palette image path.

   Ensure one pixel per color entry.

   Colors can repeat, but redundancy increases LUT generation time.
* `-s`, `--save-path <SAVE_PATH>` — Resulting LUT save path.

   If used, the resulting LUT is saved in the provided directory.

   Also used to store the optional LUT slices.
* `-l`, `--load-path <LOAD_PATH>` — LUT load path.

   The location of a bin file which stores a LUT.
* `-r`, `--resolution <RESOLUTION>` — LUT precision, number of discrete steps for each of the RGB channels.

   Use 255 for full u8 png precision.

  Default value: `16`
* `--internal-resolution <INTERNAL_RESOLUTION>` — Internal LUT precision, number of discrete steps for each of the OKLAB channels.

   Sets the resolution of the internal OKLAB LUT

  Default value: `64`
* `--save-slices` — Whether to save images of the resulting LUT.

   These slice images are saved in the LUT save path.

  Default value: `false`
* `-c`, `--convert-source-path <CONVERT_SOURCE_PATH>` — Path to a directory containing the images to convert
* `-d`, `--convert-destination-path <CONVERT_DESTINATION_PATH>` — Path to save the resulting images in.

   Created if not found.

   Defaults to the "converted" directory in the source path.
* `-n`, `--noise <NOISE>` — Additional noise to apply during conversion.

   If used, keep in the range of 0.005 to 0.025 for best results

  Default value: `0`



<hr/>