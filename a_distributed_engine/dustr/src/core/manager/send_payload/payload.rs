use serde::ser::{Serialize, SerializeStruct};

pub struct PayloadVectorInt32 {
    pub data: Vec<i32>,
}

impl Serialize for PayloadVectorInt32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Payload", 1)?;
        s.serialize_field("data", &self.data)?;
        s.end()
    }
}
