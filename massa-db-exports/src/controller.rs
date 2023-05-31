use crate::DBBatch;
use massa_models::slot::Slot;
use std::fmt::Debug;

pub trait MassaDBController: Send + Sync + Debug {
    /// Creates a new hard copy of the DB, for the given slot
    fn backup_db(&self, slot: Slot);

    /// Writes the batch to the DB
    fn write_batch(&mut self, batch: DBBatch, versioning_batch: DBBatch, change_id: Option<Slot>);

    /// Utility function to put / update a key & value in the batch
    fn put_or_update_entry_value(&self, batch: &mut DBBatch, key: Vec<u8>, value: &[u8]);

    /// Utility function to delete a key & value in the batch
    fn delete_key(&self, batch: &mut DBBatch, key: Vec<u8>);

    /// Utility function to delete all keys in a prefix
    fn delete_prefix(&mut self, prefix: &str, handle_str: &str, change_id: Option<Slot>);

    /// Reset the database, and attach it to the given slot.
    fn reset(&mut self, slot: Slot);
}
