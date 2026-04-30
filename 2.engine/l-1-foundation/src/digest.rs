use serde::{Deserialize, Serialize};

const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// Stable 64-bit digest used for canonical proof-bearing hashes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StableDigest64(pub u64);

impl StableDigest64 {
    pub const ZERO: Self = Self(0);

    pub fn builder() -> StableDigestBuilder {
        StableDigestBuilder::new()
    }

    pub fn combine(label: &[u8], digests: &[Self]) -> Self {
        let mut builder = Self::builder();
        builder.write_bytes(label);
        builder.write_u64(digests.len() as u64);
        for digest in digests {
            builder.write_u64(digest.0);
        }
        builder.finish()
    }
}

impl Default for StableDigest64 {
    fn default() -> Self {
        Self::ZERO
    }
}

/// Deterministic digest builder with explicit byte order and length prefixes.
#[derive(Debug, Clone)]
pub struct StableDigestBuilder {
    state: u64,
}

impl Default for StableDigestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StableDigestBuilder {
    pub fn new() -> Self {
        Self {
            state: FNV_OFFSET_BASIS,
        }
    }

    pub fn write_u8(&mut self, value: u8) -> &mut Self {
        self.write_raw(&[value])
    }

    pub fn write_bool(&mut self, value: bool) -> &mut Self {
        self.write_u8(u8::from(value))
    }

    pub fn write_u16(&mut self, value: u16) -> &mut Self {
        self.write_raw(&value.to_le_bytes())
    }

    pub fn write_u32(&mut self, value: u32) -> &mut Self {
        self.write_raw(&value.to_le_bytes())
    }

    pub fn write_u64(&mut self, value: u64) -> &mut Self {
        self.write_raw(&value.to_le_bytes())
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.write_u64(bytes.len() as u64);
        self.write_raw(bytes)
    }

    pub fn finish(&self) -> StableDigest64 {
        StableDigest64(self.state)
    }

    fn write_raw(&mut self, bytes: &[u8]) -> &mut Self {
        for byte in bytes {
            self.state ^= u64::from(*byte);
            self.state = self.state.wrapping_mul(FNV_PRIME);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{StableDigest64, StableDigestBuilder};

    #[test]
    fn identical_inputs_produce_identical_digest() {
        let mut left = StableDigestBuilder::new();
        left.write_bytes(b"alpha").write_u64(42);

        let mut right = StableDigestBuilder::new();
        right.write_bytes(b"alpha").write_u64(42);

        assert_eq!(left.finish(), right.finish());
    }

    #[test]
    fn byte_order_is_explicit() {
        let mut left = StableDigestBuilder::new();
        left.write_u16(0x0102);

        let mut right = StableDigestBuilder::new();
        right.write_bytes(&[0x02, 0x01]);

        assert_ne!(left.finish(), right.finish());
    }

    #[test]
    fn combine_is_stable() {
        let digests = [StableDigest64(1), StableDigest64(2), StableDigest64(3)];
        assert_eq!(
            StableDigest64::combine(b"test", &digests),
            StableDigest64::combine(b"test", &digests)
        );
    }
}
