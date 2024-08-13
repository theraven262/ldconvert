searchState.loadedDescShard("ldconvert", 0, "LUT-Diffuse Converter\nPath to save the resulting images in.\nPath to a directory containing the images to convert.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nLUT load path.\nAdditional noise to apply during conversion.\nPalette image path.\nLUT precision, number of discrete steps for each of the …\nResulting LUT save path.\nWhether to save images of the resulting LUT.\nConvertable image.\nConverts the image into a quantized image.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nName taken from the filename. The resulting image is saved …\nNoise multiplier. Added to each color during lookup.\nSaves the resulting image as a png.\nStores the relative coordinates of the color that a part …\nGenerates a LUT based on a palette, and does color …\nSame as a LUT, but can be saved using Savefile.\nStruct used to save Oklab values using Savefile crate.\nQuantizes the LUT with error diffusion.\nQuantizes a color to the nearest color to it in the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nDiffusion kernel. Stores the weights used to distribute …\nApplies the LUT to a convertible image color.\nUsed for the naming of the save file.\nInitializes fields and fills the kernel vector.\nFills the LUT with clean RGB values.\nLUT resolution.\nSaves the LUT as a binary file using the provided save …\nSaves slices of the lookup table as images using the …\nScales the color components.\nAmount of error that this kernel adds to the target color.\nNumber of steps to the target color in the red direction.\nNumber of steps to the target color in the green direction.\nNumber of steps to the target color in the blue direction.")