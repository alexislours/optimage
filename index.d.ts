/// <reference types="node" />

declare module 'optimage' {
    /**
     * Compresses an image buffer to a WebP, JPG, PNG or Avif buffer.
     * @param image_buffer The image buffer to compress.
     * @param max_dimensions The maximum width or height of the image.
     * @param [format=webp] The format of the image. Defaults to 'webp'.
     * @returns The compressed image buffer.
     */
    export function convert(image_buffer: Buffer, max_dimensions: number, format?: "webp" | "avif" | "jpg" | "png"): Buffer;
}
