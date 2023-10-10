// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

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
pub struct DeletePkU16Args {
    pub n: u16,
}

impl Reducer for DeletePkU16Args {
    const REDUCER_NAME: &'static str = "delete_pk_u16";
}

#[allow(unused)]
pub fn delete_pk_u_16(n: u16) {
    DeletePkU16Args { n }.invoke();
}

#[allow(unused)]
pub fn on_delete_pk_u_16(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u16) + Send + 'static,
) -> ReducerCallbackId<DeletePkU16Args> {
    DeletePkU16Args::on_reducer(move |__identity, __addr, __status, __args| {
        let DeletePkU16Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn once_on_delete_pk_u_16(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u16) + Send + 'static,
) -> ReducerCallbackId<DeletePkU16Args> {
    DeletePkU16Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let DeletePkU16Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn remove_on_delete_pk_u_16(id: ReducerCallbackId<DeletePkU16Args>) {
    DeletePkU16Args::remove_on_reducer(id);
}