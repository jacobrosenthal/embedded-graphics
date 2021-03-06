use nom::*;

#[derive(Debug, PartialEq)]
pub struct RlePacket<'a> {
    /// Number of pixels in this run
    pub run_length: u8,

    /// Pixel data
    pub pixel_data: &'a [u8],
}

impl<'a> RlePacket<'a> {
    /// Get the number of pixels in this packet
    pub fn len(&self) -> usize {
        self.pixel_data.len() * self.run_length as usize
    }
}

named_args!(pub rle_packet(bytes_per_pixel: u8)<&[u8], RlePacket>,
    do_parse!(
        run_length: bits!(
            preceded!(
                // 0x00 = raw packet, 0x01 = RLE packet
                tag_bits!(u8, 1, 0x01),
                // Run length is encoded as 0 = 1 pixel, 1 = 2 pixels, etc, hence this offset
                map!(take_bits!(u8, 7), |len| len + 1)
            )
        ) >>
        pixel_data: take!(bytes_per_pixel) >>
        (
            RlePacket {
                // Run length is encoded as 0 = 1 pixel, 1 = 2 pixels, etc, hence this offset
                run_length,
                pixel_data
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = [
            // 2 pixels worth of RLE data
            0b1000_0001,
            // 32BPP pixel
            0xAA,
            0xBB,
            0xCC,
            0xDD,
        ];

        let (remaining, packet) = rle_packet(&input, 4).unwrap();

        assert_eq!(remaining, &[]);
        assert_eq!(
            packet,
            RlePacket {
                run_length: 2,
                pixel_data: &[0xAA, 0xBB, 0xCC, 0xDD]
            }
        );
    }

    #[test]
    fn ignore_raw_packet() {
        let input = [
            // 2 pixels worth of raw (non-RLE) data
            0b0000_0001,
            // 32BPP pixel
            0xAA,
            0xBB,
            0xCC,
            0xDD,
        ];

        let result = rle_packet(&input, 4);

        assert!(result.is_err());
    }

    #[test]
    fn stop_at_packet_end() {
        let input = [
            // 2 pixels worth of RLE data
            0b1000_0001,
            // 32BPP pixel
            0xAA,
            0xBB,
            0xCC,
            0xDD,
            // 32BPP pixel
            0x11,
            0x22,
            0x33,
            0x44,
        ];

        let (remaining, packet) = rle_packet(&input, 4).unwrap();

        assert_eq!(remaining, &[0x11, 0x22, 0x33, 0x44]);
        assert_eq!(
            packet,
            RlePacket {
                run_length: 2,
                pixel_data: &[0xAA, 0xBB, 0xCC, 0xDD]
            }
        );
    }
}
