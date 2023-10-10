// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

use super::every_primitive_struct::EveryPrimitiveStruct;
#[allow(unused)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OneEveryPrimitiveStruct {
    pub s: EveryPrimitiveStruct,
}

impl TableType for OneEveryPrimitiveStruct {
    const TABLE_NAME: &'static str = "OneEveryPrimitiveStruct";
    type ReducerEvent = super::ReducerEvent;
}

impl OneEveryPrimitiveStruct {
    #[allow(unused)]
    pub fn filter_by_s(s: EveryPrimitiveStruct) -> TableIter<Self> {
        Self::filter(|row| row.s == s)
    }
}