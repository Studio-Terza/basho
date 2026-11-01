#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProtoBytes(Vec<u8>);

impl ProtoBytes {
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    pub fn number_field(&mut self, field: u64, number: u64) {
        self.varint(field << 3);
        self.varint(number);
    }

    pub fn bytes_field(&mut self, field: u64, body: &[u8]) {
        self.varint((field << 3) | 2);
        self.varint(u64::try_from(body.len()).unwrap_or(u64::MAX));
        self.0.extend_from_slice(body);
    }

    pub fn append(&mut self, bytes: &[u8]) {
        self.0.extend_from_slice(bytes);
    }

    #[expect(
        clippy::needless_for_each,
        reason = "expression-flow drives iterators with combinators, never a for loop"
    )]
    #[must_use]
    pub fn packed(numbers: &[u64]) -> Vec<u8> {
        let mut bytes = ProtoBytes::default();
        numbers.iter().for_each(|number| bytes.varint(*number));
        bytes.0
    }

    fn varint(&mut self, number: u64) {
        let parts: Vec<u8> = std::iter::successors(Some(number), |rest| {
            Some(rest >> 7).filter(|rest| *rest > 0)
        })
        .map(|rest| rest.to_le_bytes()[0])
        .collect();
        let last = parts.len().saturating_sub(1);
        self.0.extend(parts.iter().enumerate().map(|(index, part)| {
            if index == last {
                part & 0x7f
            } else {
                part | 0x80
            }
        }));
    }
}
