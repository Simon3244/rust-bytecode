use std::io::Read;

use crate::Result;
use byteorder::{BigEndian, ReadBytesExt};

use crate::{
    structs::{ConstItem, ConstPool},
    Readable,
};

impl Readable for ConstPool {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let count = reader.read_u16::<BigEndian>()?;
        let mut entries = Vec::with_capacity(count as usize);
        entries.push(None);
        let mut skip = false;
        for _ in 1..count {
            if skip {
                skip = false;
                continue;
            }

            let entry = ConstItem::read(reader)?;

            if let ConstItem::Long(_) | ConstItem::Double(_) = entry {
                entries.push(None);
                skip = true;
            }

            entries.push(Some(entry));
        }
        Ok(ConstPool { entries })
    }
}
