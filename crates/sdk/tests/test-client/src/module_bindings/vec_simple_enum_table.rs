// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::simple_enum_type::SimpleEnum;
use super::vec_simple_enum_type::VecSimpleEnum;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `vec_simple_enum`.
///
/// Obtain a handle from the [`VecSimpleEnumTableAccess::vec_simple_enum`] method on [`super::RemoteTables`],
/// like `ctx.db.vec_simple_enum()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.vec_simple_enum().on_insert(...)`.
pub struct VecSimpleEnumTableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<VecSimpleEnum>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `vec_simple_enum`.
///
/// Implemented for [`super::RemoteTables`].
pub trait VecSimpleEnumTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`VecSimpleEnumTableHandle`], which mediates access to the table `vec_simple_enum`.
    fn vec_simple_enum(&self) -> VecSimpleEnumTableHandle<'_>;
}

impl VecSimpleEnumTableAccess for super::RemoteTables {
    fn vec_simple_enum(&self) -> VecSimpleEnumTableHandle<'_> {
        VecSimpleEnumTableHandle {
            imp: self.imp.get_table::<VecSimpleEnum>("vec_simple_enum"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct VecSimpleEnumInsertCallbackId(__sdk::callbacks::CallbackId);
pub struct VecSimpleEnumDeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for VecSimpleEnumTableHandle<'ctx> {
    type Row = VecSimpleEnum;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = VecSimpleEnum> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = VecSimpleEnumInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecSimpleEnumInsertCallbackId {
        VecSimpleEnumInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: VecSimpleEnumInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = VecSimpleEnumDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> VecSimpleEnumDeleteCallbackId {
        VecSimpleEnumDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: VecSimpleEnumDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<VecSimpleEnum>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(raw_updates)
        .context("Failed to parse table update for table \"vec_simple_enum\"")
}