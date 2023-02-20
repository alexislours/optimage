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

// ES Modules
import { convert } from "@alexislours/optimage";

const imageBuffer = fs.readFileSync("image.png");

// Will convert the image to webp and
// resize it to 1000px width or height
// whichever is the biggest
const convertedBuffer = convert(imageBuffer, 1000, "webp");
```

```ts
/**
 * Compresses an image buffer to a WebP, JPG, PNG or Avif buffer.
 * @param image_buffer The image buffer to compress.
 * @param max_dimensions The maximum width or height of the image.
 * @param [format=webp] The format of the image. Defaults to 'webp'.
 * @returns The compressed image buffer.
 */
export function convert(
  image_buffer: Buffer,
  max_dimensions: number,
  format?: "webp" | "avif" | "jpg" | "png"
): Buffer;
```
