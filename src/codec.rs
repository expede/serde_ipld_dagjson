use std::io::{BufRead, Write};

use ipld_core::{
    cid::Cid,
    codec::{Codec, Links},
    serde::ExtractLinks,
};

use serde::{de::Deserialize, ser::Serialize};

use crate::{de::Deserializer, error::CodecError};

/// DAG-JSON implementation of ipld-core's `Codec` trait.
pub struct DagJsonCodec;

const RAW_CODE: u64 = 0x129;

impl<T> Codec<T> for DagJsonCodec
where
    T: for<'a> Deserialize<'a> + Serialize,
{
    type Error = CodecError;

    fn to_code(&self) -> u64 {
        RAW_CODE
    }

    fn try_from_code(code: u64) -> Option<Self> {
        match code {
            RAW_CODE => Some(DagJsonCodec),
            _ => None
        }
    }

    fn decode<R: BufRead>(&self, reader: R) -> Result<T, Self::Error> {
        Ok(crate::from_reader(reader)?)
    }

    fn encode<W: Write>(&self, writer: W, data: &T) -> Result<(), Self::Error> {
        Ok(crate::to_writer(writer, data)?)
    }
}

impl Links for DagJsonCodec {
    type LinksError = CodecError;

    fn links(&self, data: &[u8]) -> Result<impl Iterator<Item = Cid>, Self::LinksError> {
        let mut json_deserializer = serde_json::Deserializer::from_slice(data);
        let deserializer = Deserializer::new(&mut json_deserializer);
        Ok(ExtractLinks::deserialize(deserializer)?
            .into_vec()
            .into_iter())
    }
}

impl From<DagJsonCodec> for u64 {
    fn from(_: DagJsonCodec) -> u64 {
        RAW_CODE
    }
}

impl TryFrom<u64> for DagJsonCodec {
    type Error = NotDagJsonCode;

    fn try_from(code: u64) -> Result<Self, Self::Error> {
        if code == RAW_CODE {
            Ok(DagJsonCodec)
        } else {
            Err(NotDagJsonCode(code))
        }
    }
}

/// FIXME
pub struct NotDagJsonCode(pub u64);
