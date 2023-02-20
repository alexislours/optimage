![NPM](https://img.shields.io/npm/l/@alexislours/optimage) ![npm (scoped)](https://img.shields.io/npm/v/@alexislours/optimage)

# @alexislours/optimage

**@alexislours/optimage:** A native module to convert and optimize image buffers.

## Installing @alexislours/optimage

```sh
$ npm install @alexislours/optimage
```

Prebuilt binaries are available for Linux x86 and macOS ARM64. If you are using a different platform, you will need to have the Rust toolchain installed.

## Usage

optimage exposes a single function, `convert`, which takes an image buffer and returns the modified image buffer.

```js
// CommonJS
const { convert } = require("@alexislours/optimage");
const fs = require("node:fs");

// ES Modules
import { convert } from "@alexislours/optimage";
import fs from "node:fs";

const imageBuffer = fs.readFileSync("image.png");

// Will convert the image to webp and
// resize it to 1000px width or height
// whichever is the biggest and use
// the lanczos3 filter for resizing.
const convertedBuffer = convert({
  buffer: imageBuffer,
  max_dimensions: 1000,
  format: "webp",
  filter: "lanczos3",
});
```

```ts
/**
 * The configuration object for the convert function.
 */
type ConvertConfig = {
  /**
   * The image buffer to compress.
   */
  buffer: Buffer;
  /**
   * The maximum width or height of the converted image.
   */
  max_dimensions: number;
  /**
   * The format of the converted image. Defaults to webp.
   * @default "webp"
   */
  format?: Format;
  /**
   * The filter to use when resizing the image. Defaults to nearest.
   * @default "nearest"
   */
  filter?: FilterType;
};

type Format = "webp" | "avif" | "jpg" | "png" | "tiff" | "gif" | "bmp" | "tga";

type FilterType =
  | "nearest"
  | "lanczos3"
  | "gaussian"
  | "triangle"
  | "catmullrom";

/**
 * Converts an image buffer to a resized and compressed image buffer.
 * @param convertConfig The configuration object.
 * @returns The compressed image buffer.
 */
export function convert(convertConfig: ConvertConfig): Buffer;
```
