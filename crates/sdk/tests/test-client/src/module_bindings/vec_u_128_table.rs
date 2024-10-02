// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::vec_u_128_type::VecU128;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `vec_u128`.
///
/// Obtain a handle from the [`VecU128TableAccess::vec_u_128`] method on [`super::RemoteTables`],
/// like `ctx.db.vec_u_128()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.vec_u_128().on_insert(...)`.
pub struct VecU128TableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<VecU128>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `vec_u128`.
///
/// Implemented for [`super::RemoteTables`].
pub trait VecU128TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`VecU128TableHandle`], which mediates access to the table `vec_u128`.
    fn vec_u_128(&self) -> VecU128TableHandle<'_>;
}

impl VecU128TableAccess for super::RemoteTables {
    fn vec_u_128(&self) -> VecU128TableHandle<'_> {
        VecU128TableHandle {
            imp: self.imp.get_table::<VecU128>("vec_u128"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct VecU128InsertCallbackId(__sdk::callbacks::CallbackId);
pub struct VecU128DeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for VecU128TableHandle<'ctx> {
    type Row = VecU128;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = VecU128> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = VecU128InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecU128InsertCallbackId {
        VecU128InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: VecU128InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = VecU128DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecU128DeleteCallbackId {
        VecU128DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: VecU128DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<VecU128>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .context("Failed to parse table update for table \"vec_u128\"")
}