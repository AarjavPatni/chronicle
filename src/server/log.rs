#[derive(Clone)]
pub struct Record {
    pub value: Vec<u8>,
    pub offset: u64,
}

pub struct Log {
    pub records: Vec<Record>,
}

impl Log {
    pub fn new_log() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn append(&mut self, mut record: Record) -> Result<u64, String> {
        let offset: u64 = self.records.len() as u64;
        record.offset = offset;
        self.records.push(record);
        Ok(offset)
    }

    pub fn read(&self, offset: u64) -> Result<Record, String> {
        let max_size: u64 = self.records.len() as u64;

        if offset >= max_size {
            return Err(String::from("Offset exceeded length of Log"));
        }

        Ok(self.records[offset as usize].clone())
    }
}
