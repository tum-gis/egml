use std::collections::LinkedList;
use std::io::Write;

#[derive(Default)]
pub struct ChunkedSink(LinkedList<Vec<u8>>);

impl ChunkedSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
    }

    /// Streams every accumulated chunk to `sink` in order. This is where
    /// the copies deferred by `write`/`append` finally happen — one per
    /// chunk, the same unavoidable cost as writing directly to `sink` — but
    /// paid exactly once, not once per splice.
    pub fn write_to<W: Write>(&self, sink: &mut W) -> std::io::Result<()> {
        for chunk in &self.0 {
            sink.write_all(chunk)?;
        }
        Ok(())
    }
}

impl Write for ChunkedSink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.0.back_mut() {
            Some(chunk) => chunk.extend_from_slice(buf),
            None => self.0.push_back(buf.to_vec()),
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
