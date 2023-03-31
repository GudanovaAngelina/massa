(function() {var implementors = {
"massa_async_pool":[["impl Deserializer&lt;<a class=\"struct\" href=\"massa_async_pool/struct.AsyncMessage.html\" title=\"struct massa_async_pool::AsyncMessage\">AsyncMessage</a>&gt; for <a class=\"struct\" href=\"massa_async_pool/struct.AsyncMessageDeserializer.html\" title=\"struct massa_async_pool::AsyncMessageDeserializer\">AsyncMessageDeserializer</a>"],["impl Deserializer&lt;(<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/cmp/struct.Reverse.html\" title=\"struct core::cmp::Reverse\">Reverse</a>&lt;<a class=\"struct\" href=\"https://docs.rs/num-rational/0.4/num_rational/struct.Ratio.html\" title=\"struct num_rational::Ratio\">Ratio</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt;&gt;, Slot, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>)&gt; for <a class=\"struct\" href=\"massa_async_pool/struct.AsyncMessageIdDeserializer.html\" title=\"struct massa_async_pool::AsyncMessageIdDeserializer\">AsyncMessageIdDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;(<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/cmp/struct.Reverse.html\" title=\"struct core::cmp::Reverse\">Reverse</a>&lt;<a class=\"struct\" href=\"https://docs.rs/num-rational/0.4/num_rational/struct.Ratio.html\" title=\"struct num_rational::Ratio\">Ratio</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt;&gt;, Slot, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>), <a class=\"struct\" href=\"massa_async_pool/struct.AsyncMessage.html\" title=\"struct massa_async_pool::AsyncMessage\">AsyncMessage</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_async_pool/struct.AsyncPoolDeserializer.html\" title=\"struct massa_async_pool::AsyncPoolDeserializer\">AsyncPoolDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_async_pool/struct.AsyncPoolChanges.html\" title=\"struct massa_async_pool::AsyncPoolChanges\">AsyncPoolChanges</a>&gt; for <a class=\"struct\" href=\"massa_async_pool/struct.AsyncPoolChangesDeserializer.html\" title=\"struct massa_async_pool::AsyncPoolChangesDeserializer\">AsyncPoolChangesDeserializer</a>"]],
"massa_bootstrap":[["impl Deserializer&lt;<a class=\"enum\" href=\"massa_bootstrap/enum.BootstrapClientMessage.html\" title=\"enum massa_bootstrap::BootstrapClientMessage\">BootstrapClientMessage</a>&gt; for <a class=\"struct\" href=\"massa_bootstrap/struct.BootstrapClientMessageDeserializer.html\" title=\"struct massa_bootstrap::BootstrapClientMessageDeserializer\">BootstrapClientMessageDeserializer</a>"],["impl Deserializer&lt;<a class=\"enum\" href=\"massa_bootstrap/enum.BootstrapServerMessage.html\" title=\"enum massa_bootstrap::BootstrapServerMessage\">BootstrapServerMessage</a>&gt; for <a class=\"struct\" href=\"massa_bootstrap/struct.BootstrapServerMessageDeserializer.html\" title=\"struct massa_bootstrap::BootstrapServerMessageDeserializer\">BootstrapServerMessageDeserializer</a>"]],
"massa_consensus_exports":[["impl Deserializer&lt;<a class=\"struct\" href=\"massa_consensus_exports/bootstrapable_graph/struct.BootstrapableGraph.html\" title=\"struct massa_consensus_exports::bootstrapable_graph::BootstrapableGraph\">BootstrapableGraph</a>&gt; for <a class=\"struct\" href=\"massa_consensus_exports/bootstrapable_graph/struct.BootstrapableGraphDeserializer.html\" title=\"struct massa_consensus_exports::bootstrapable_graph::BootstrapableGraphDeserializer\">BootstrapableGraphDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_consensus_exports/export_active_block/struct.ExportActiveBlock.html\" title=\"struct massa_consensus_exports::export_active_block::ExportActiveBlock\">ExportActiveBlock</a>&gt; for <a class=\"struct\" href=\"massa_consensus_exports/export_active_block/struct.ExportActiveBlockDeserializer.html\" title=\"struct massa_consensus_exports::export_active_block::ExportActiveBlockDeserializer\">ExportActiveBlockDeserializer</a>"]],
"massa_executed_ops":[["impl Deserializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;Slot, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html\" title=\"struct std::collections::hash::set::HashSet\">HashSet</a>&lt;OperationId, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;HashMapper&lt;OperationId&gt;&gt;&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_executed_ops/struct.ExecutedOpsDeserializer.html\" title=\"struct massa_executed_ops::ExecutedOpsDeserializer\">ExecutedOpsDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html\" title=\"struct std::collections::hash::map::HashMap\">HashMap</a>&lt;OperationId, (<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>, Slot), <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;HashMapper&lt;OperationId&gt;&gt;&gt;&gt; for <a class=\"struct\" href=\"massa_executed_ops/struct.ExecutedOpsChangesDeserializer.html\" title=\"struct massa_executed_ops::ExecutedOpsChangesDeserializer\">ExecutedOpsChangesDeserializer</a>"]],
"massa_final_state":[["impl Deserializer&lt;<a class=\"struct\" href=\"massa_final_state/struct.StateChanges.html\" title=\"struct massa_final_state::StateChanges\">StateChanges</a>&gt; for <a class=\"struct\" href=\"massa_final_state/struct.StateChangesDeserializer.html\" title=\"struct massa_final_state::StateChangesDeserializer\">StateChangesDeserializer</a>"]],
"massa_hash":[["impl Deserializer&lt;<a class=\"struct\" href=\"massa_hash/struct.Hash.html\" title=\"struct massa_hash::Hash\">Hash</a>&gt; for <a class=\"struct\" href=\"massa_hash/struct.HashDeserializer.html\" title=\"struct massa_hash::HashDeserializer\">HashDeserializer</a>"]],
"massa_ledger_exports":[["impl Deserializer&lt;<a class=\"struct\" href=\"massa_ledger_exports/struct.LedgerEntry.html\" title=\"struct massa_ledger_exports::LedgerEntry\">LedgerEntry</a>&gt; for <a class=\"struct\" href=\"massa_ledger_exports/struct.LedgerEntryDeserializer.html\" title=\"struct massa_ledger_exports::LedgerEntryDeserializer\">LedgerEntryDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_ledger_exports/struct.Key.html\" title=\"struct massa_ledger_exports::Key\">Key</a>&gt; for <a class=\"struct\" href=\"massa_ledger_exports/struct.KeyDeserializer.html\" title=\"struct massa_ledger_exports::KeyDeserializer\">KeyDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_ledger_exports/struct.LedgerEntryUpdate.html\" title=\"struct massa_ledger_exports::LedgerEntryUpdate\">LedgerEntryUpdate</a>&gt; for <a class=\"struct\" href=\"massa_ledger_exports/struct.LedgerEntryUpdateDeserializer.html\" title=\"struct massa_ledger_exports::LedgerEntryUpdateDeserializer\">LedgerEntryUpdateDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_ledger_exports/struct.LedgerChanges.html\" title=\"struct massa_ledger_exports::LedgerChanges\">LedgerChanges</a>&gt; for <a class=\"struct\" href=\"massa_ledger_exports/struct.LedgerChangesDeserializer.html\" title=\"struct massa_ledger_exports::LedgerChangesDeserializer\">LedgerChangesDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;, <a class=\"enum\" href=\"massa_ledger_exports/enum.SetOrDelete.html\" title=\"enum massa_ledger_exports::SetOrDelete\">SetOrDelete</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_ledger_exports/struct.DatastoreUpdateDeserializer.html\" title=\"struct massa_ledger_exports::DatastoreUpdateDeserializer\">DatastoreUpdateDeserializer</a>"]],
"massa_models":[["impl Deserializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"massa_models/secure_share/struct.SecureShare.html\" title=\"struct massa_models::secure_share::SecureShare\">SecureShare</a>&lt;<a class=\"struct\" href=\"massa_models/operation/struct.Operation.html\" title=\"struct massa_models::operation::Operation\">Operation</a>, <a class=\"struct\" href=\"massa_models/operation/struct.OperationId.html\" title=\"struct massa_models::operation::OperationId\">OperationId</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationsDeserializer.html\" title=\"struct massa_models::operation::OperationsDeserializer\">OperationsDeserializer</a>"],["impl Deserializer&lt;<a class=\"enum\" href=\"massa_models/operation/enum.OperationType.html\" title=\"enum massa_models::operation::OperationType\">OperationType</a>&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationTypeDeserializer.html\" title=\"struct massa_models::operation::OperationTypeDeserializer\">OperationTypeDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/endorsement/struct.Endorsement.html\" title=\"struct massa_models::endorsement::Endorsement\">Endorsement</a>&gt; for <a class=\"struct\" href=\"massa_models/endorsement/struct.EndorsementDeserializer.html\" title=\"struct massa_models::endorsement::EndorsementDeserializer\">EndorsementDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/ledger/struct.LedgerChange.html\" title=\"struct massa_models::ledger::LedgerChange\">LedgerChange</a>&gt; for <a class=\"struct\" href=\"massa_models/ledger/struct.LedgerChangeDeserializer.html\" title=\"struct massa_models::ledger::LedgerChangeDeserializer\">LedgerChangeDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/clique/struct.Clique.html\" title=\"struct massa_models::clique::Clique\">Clique</a>&gt; for <a class=\"struct\" href=\"massa_models/clique/struct.CliqueDeserializer.html\" title=\"struct massa_models::clique::CliqueDeserializer\">CliqueDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/operation/struct.OperationId.html\" title=\"struct massa_models::operation::OperationId\">OperationId</a>&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationIdDeserializer.html\" title=\"struct massa_models::operation::OperationIdDeserializer\">OperationIdDeserializer</a>"],["impl&lt;T, ID, Deser&gt; Deserializer&lt;<a class=\"struct\" href=\"massa_models/secure_share/struct.SecureShare.html\" title=\"struct massa_models::secure_share::SecureShare\">SecureShare</a>&lt;T, ID&gt;&gt; for <a class=\"struct\" href=\"massa_models/secure_share/struct.SecureShareDeserializer.html\" title=\"struct massa_models::secure_share::SecureShareDeserializer\">SecureShareDeserializer</a>&lt;T, Deser&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"massa_models/secure_share/trait.SecureShareContent.html\" title=\"trait massa_models::secure_share::SecureShareContent\">SecureShareContent</a>,\n    ID: <a class=\"trait\" href=\"massa_models/secure_share/trait.Id.html\" title=\"trait massa_models::secure_share::Id\">Id</a>,\n    Deser: Deserializer&lt;T&gt;,</span>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/block_header/struct.BlockHeader.html\" title=\"struct massa_models::block_header::BlockHeader\">BlockHeader</a>&gt; for <a class=\"struct\" href=\"massa_models/block_header/struct.BlockHeaderDeserializer.html\" title=\"struct massa_models::block_header::BlockHeaderDeserializer\">BlockHeaderDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/bytecode/struct.Bytecode.html\" title=\"struct massa_models::bytecode::Bytecode\">Bytecode</a>&gt; for <a class=\"struct\" href=\"massa_models/bytecode/struct.BytecodeDeserializer.html\" title=\"struct massa_models::bytecode::BytecodeDeserializer\">BytecodeDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/operation/struct.OperationPrefixId.html\" title=\"struct massa_models::operation::OperationPrefixId\">OperationPrefixId</a>&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationPrefixIdDeserializer.html\" title=\"struct massa_models::operation::OperationPrefixIdDeserializer\">OperationPrefixIdDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_models/datastore/struct.DatastoreDeserializer.html\" title=\"struct massa_models::datastore::DatastoreDeserializer\">DatastoreDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html\" title=\"struct std::collections::hash::set::HashSet\">HashSet</a>&lt;<a class=\"struct\" href=\"massa_models/operation/struct.OperationPrefixId.html\" title=\"struct massa_models::operation::OperationPrefixId\">OperationPrefixId</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;<a class=\"struct\" href=\"massa_models/prehash/struct.HashMapper.html\" title=\"struct massa_models::prehash::HashMapper\">HashMapper</a>&lt;<a class=\"struct\" href=\"massa_models/operation/struct.OperationPrefixId.html\" title=\"struct massa_models::operation::OperationPrefixId\">OperationPrefixId</a>&gt;&gt;&gt;&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationPrefixIdsDeserializer.html\" title=\"struct massa_models::operation::OperationPrefixIdsDeserializer\">OperationPrefixIdsDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/ledger/struct.LedgerChanges.html\" title=\"struct massa_models::ledger::LedgerChanges\">LedgerChanges</a>&gt; for <a class=\"struct\" href=\"massa_models/ledger/struct.LedgerChangesDeserializer.html\" title=\"struct massa_models::ledger::LedgerChangesDeserializer\">LedgerChangesDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/block_id/struct.BlockId.html\" title=\"struct massa_models::block_id::BlockId\">BlockId</a>&gt; for <a class=\"struct\" href=\"massa_models/block_id/struct.BlockIdDeserializer.html\" title=\"struct massa_models::block_id::BlockIdDeserializer\">BlockIdDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/ledger/struct.LedgerData.html\" title=\"struct massa_models::ledger::LedgerData\">LedgerData</a>&gt; for <a class=\"struct\" href=\"massa_models/ledger/struct.LedgerDataDeserializer.html\" title=\"struct massa_models::ledger::LedgerDataDeserializer\">LedgerDataDeserializer</a>"],["impl Deserializer&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/std/net/ip_addr/enum.IpAddr.html\" title=\"enum std::net::ip_addr::IpAddr\">IpAddr</a>&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.IpAddrDeserializer.html\" title=\"struct massa_models::serialization::IpAddrDeserializer\">IpAddrDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/version/struct.Version.html\" title=\"struct massa_models::version::Version\">Version</a>&gt; for <a class=\"struct\" href=\"massa_models/version/struct.VersionDeserializer.html\" title=\"struct massa_models::version::VersionDeserializer\">VersionDeserializer</a>"],["impl&lt;T, ST&gt; Deserializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;T, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.VecDeserializer.html\" title=\"struct massa_models::serialization::VecDeserializer\">VecDeserializer</a>&lt;T, ST&gt;<span class=\"where fmt-newline\">where\n    ST: Deserializer&lt;T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/rolls/struct.RollUpdate.html\" title=\"struct massa_models::rolls::RollUpdate\">RollUpdate</a>&gt; for <a class=\"struct\" href=\"massa_models/rolls/struct.RollUpdateDeserializer.html\" title=\"struct massa_models::rolls::RollUpdateDeserializer\">RollUpdateDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/block/struct.Block.html\" title=\"struct massa_models::block::Block\">Block</a>&gt; for <a class=\"struct\" href=\"massa_models/block/struct.BlockDeserializer.html\" title=\"struct massa_models::block::BlockDeserializer\">BlockDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/operation/struct.Operation.html\" title=\"struct massa_models::operation::Operation\">Operation</a>&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationDeserializer.html\" title=\"struct massa_models::operation::OperationDeserializer\">OperationDeserializer</a>"],["impl&lt;T, ST&gt; Deserializer&lt;<a class=\"enum\" href=\"massa_models/streaming_step/enum.StreamingStep.html\" title=\"enum massa_models::streaming_step::StreamingStep\">StreamingStep</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"massa_models/streaming_step/struct.StreamingStepDeserializer.html\" title=\"struct massa_models::streaming_step::StreamingStepDeserializer\">StreamingStepDeserializer</a>&lt;T, ST&gt;<span class=\"where fmt-newline\">where\n    ST: Deserializer&lt;T&gt;,\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/amount/struct.Amount.html\" title=\"struct massa_models::amount::Amount\">Amount</a>&gt; for <a class=\"struct\" href=\"massa_models/amount/struct.AmountDeserializer.html\" title=\"struct massa_models::amount::AmountDeserializer\">AmountDeserializer</a>"],["impl&lt;DL, L&gt; Deserializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.StringDeserializer.html\" title=\"struct massa_models::serialization::StringDeserializer\">StringDeserializer</a>&lt;DL, L&gt;<span class=\"where fmt-newline\">where\n    DL: Deserializer&lt;L&gt;,\n    L: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; + ToUsize,</span>"],["impl Deserializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.VecU8Deserializer.html\" title=\"struct massa_models::serialization::VecU8Deserializer\">VecU8Deserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/slot/struct.Slot.html\" title=\"struct massa_models::slot::Slot\">Slot</a>&gt; for <a class=\"struct\" href=\"massa_models/slot/struct.SlotDeserializer.html\" title=\"struct massa_models::slot::SlotDeserializer\">SlotDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_models/endorsement/struct.Endorsement.html\" title=\"struct massa_models::endorsement::Endorsement\">Endorsement</a>&gt; for <a class=\"struct\" href=\"massa_models/endorsement/struct.EndorsementDeserializerLW.html\" title=\"struct massa_models::endorsement::EndorsementDeserializerLW\">EndorsementDeserializerLW</a>"],["impl Deserializer&lt;BitVec&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, Lsb0&gt;&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.BitVecDeserializer.html\" title=\"struct massa_models::serialization::BitVecDeserializer\">BitVecDeserializer</a>"],["impl&lt;T, ST&gt; Deserializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html\" title=\"struct std::collections::hash::set::HashSet\">HashSet</a>&lt;T, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;<a class=\"struct\" href=\"massa_models/prehash/struct.HashMapper.html\" title=\"struct massa_models::prehash::HashMapper\">HashMapper</a>&lt;T&gt;&gt;&gt;&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.PreHashSetDeserializer.html\" title=\"struct massa_models::serialization::PreHashSetDeserializer\">PreHashSetDeserializer</a>&lt;T, ST&gt;<span class=\"where fmt-newline\">where\n    ST: Deserializer&lt;T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    T: <a class=\"trait\" href=\"massa_models/prehash/trait.PreHashed.html\" title=\"trait massa_models::prehash::PreHashed\">PreHashed</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,</span>"],["impl Deserializer&lt;<a class=\"enum\" href=\"massa_models/address/enum.Address.html\" title=\"enum massa_models::address::Address\">Address</a>&gt; for <a class=\"struct\" href=\"massa_models/address/struct.AddressDeserializer.html\" title=\"struct massa_models::address::AddressDeserializer\">AddressDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"massa_models/operation/struct.OperationId.html\" title=\"struct massa_models::operation::OperationId\">OperationId</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationIdsDeserializer.html\" title=\"struct massa_models::operation::OperationIdsDeserializer\">OperationIdsDeserializer</a>"]],
"massa_network_exports":[["impl Deserializer&lt;<a class=\"struct\" href=\"massa_network_exports/struct.BootstrapPeers.html\" title=\"struct massa_network_exports::BootstrapPeers\">BootstrapPeers</a>&gt; for <a class=\"struct\" href=\"massa_network_exports/struct.BootstrapPeersDeserializer.html\" title=\"struct massa_network_exports::BootstrapPeersDeserializer\">BootstrapPeersDeserializer</a>"]],
"massa_pos_exports":[["impl Deserializer&lt;<a class=\"struct\" href=\"massa_pos_exports/struct.DeferredCredits.html\" title=\"struct massa_pos_exports::DeferredCredits\">DeferredCredits</a>&gt; for <a class=\"struct\" href=\"massa_pos_exports/struct.DeferredCreditsDeserializer.html\" title=\"struct massa_pos_exports::DeferredCreditsDeserializer\">DeferredCreditsDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_pos_exports/struct.PoSChanges.html\" title=\"struct massa_pos_exports::PoSChanges\">PoSChanges</a>&gt; for <a class=\"struct\" href=\"massa_pos_exports/struct.PoSChangesDeserializer.html\" title=\"struct massa_pos_exports::PoSChangesDeserializer\">PoSChangesDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_pos_exports/struct.CycleInfo.html\" title=\"struct massa_pos_exports::CycleInfo\">CycleInfo</a>&gt; for <a class=\"struct\" href=\"massa_pos_exports/struct.CycleInfoDeserializer.html\" title=\"struct massa_pos_exports::CycleInfoDeserializer\">CycleInfoDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;(Address, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>), <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_pos_exports/struct.RollsDeserializer.html\" title=\"struct massa_pos_exports::RollsDeserializer\">RollsDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html\" title=\"struct std::collections::hash::map::HashMap\">HashMap</a>&lt;Address, <a class=\"struct\" href=\"massa_pos_exports/struct.ProductionStats.html\" title=\"struct massa_pos_exports::ProductionStats\">ProductionStats</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;HashMapper&lt;Address&gt;&gt;&gt;&gt; for <a class=\"struct\" href=\"massa_pos_exports/struct.ProductionStatsDeserializer.html\" title=\"struct massa_pos_exports::ProductionStatsDeserializer\">ProductionStatsDeserializer</a>"]],
"massa_serialization":[],
"massa_signature":[["impl Deserializer&lt;<a class=\"struct\" href=\"massa_signature/struct.Signature.html\" title=\"struct massa_signature::Signature\">Signature</a>&gt; for <a class=\"struct\" href=\"massa_signature/struct.SignatureDeserializer.html\" title=\"struct massa_signature::SignatureDeserializer\">SignatureDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_signature/struct.PublicKey.html\" title=\"struct massa_signature::PublicKey\">PublicKey</a>&gt; for <a class=\"struct\" href=\"massa_signature/struct.PublicKeyDeserializer.html\" title=\"struct massa_signature::PublicKeyDeserializer\">PublicKeyDeserializer</a>"]],
"massa_time":[["impl Deserializer&lt;<a class=\"struct\" href=\"massa_time/struct.MassaTime.html\" title=\"struct massa_time::MassaTime\">MassaTime</a>&gt; for <a class=\"struct\" href=\"massa_time/struct.MassaTimeDeserializer.html\" title=\"struct massa_time::MassaTimeDeserializer\">MassaTimeDeserializer</a>"]],
"massa_versioning_worker":[["impl Deserializer&lt;<a class=\"struct\" href=\"massa_versioning_worker/versioning/struct.Advance.html\" title=\"struct massa_versioning_worker::versioning::Advance\">Advance</a>&gt; for <a class=\"struct\" href=\"massa_versioning_worker/versioning_ser_der/struct.AdvanceDeserializer.html\" title=\"struct massa_versioning_worker::versioning_ser_der::AdvanceDeserializer\">AdvanceDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_versioning_worker/versioning/struct.MipState.html\" title=\"struct massa_versioning_worker::versioning::MipState\">MipState</a>&gt; for <a class=\"struct\" href=\"massa_versioning_worker/versioning_ser_der/struct.MipStateDeserializer.html\" title=\"struct massa_versioning_worker::versioning_ser_der::MipStateDeserializer\">MipStateDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_versioning_worker/versioning/struct.MipStoreRaw.html\" title=\"struct massa_versioning_worker::versioning::MipStoreRaw\">MipStoreRaw</a>&gt; for <a class=\"struct\" href=\"massa_versioning_worker/versioning_ser_der/struct.MipStoreRawDeserializer.html\" title=\"struct massa_versioning_worker::versioning_ser_der::MipStoreRawDeserializer\">MipStoreRawDeserializer</a>"],["impl Deserializer&lt;<a class=\"struct\" href=\"massa_versioning_worker/versioning/struct.MipInfo.html\" title=\"struct massa_versioning_worker::versioning::MipInfo\">MipInfo</a>&gt; for <a class=\"struct\" href=\"massa_versioning_worker/versioning_ser_der/struct.MipInfoDeserializer.html\" title=\"struct massa_versioning_worker::versioning_ser_der::MipInfoDeserializer\">MipInfoDeserializer</a>"],["impl Deserializer&lt;<a class=\"enum\" href=\"massa_versioning_worker/versioning/enum.ComponentState.html\" title=\"enum massa_versioning_worker::versioning::ComponentState\">ComponentState</a>&gt; for <a class=\"struct\" href=\"massa_versioning_worker/versioning_ser_der/struct.ComponentStateDeserializer.html\" title=\"struct massa_versioning_worker::versioning_ser_der::ComponentStateDeserializer\">ComponentStateDeserializer</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()