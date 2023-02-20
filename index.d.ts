/// <reference types="node" />
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
}

type Format =
  | "webp"
  | "avif"
  | "jpg"
  | "png"
  | "tiff"
  | "gif"
  | "bmp"
  | "tga";

type FilterType =
  | "nearest"
  | "lanczos"
  | "gaussian"
  | "triangle"
  | "catmullrom"

declare module "@alexislours/optimage" {
  /**
   * Converts an image buffer to a resized and compressed image buffer.
   * @param convertConfig The configuration object.
   * @returns The compressed image buffer.
   */
  export function convert(convertConfig: ConvertConfig): Buffer;
}
