(function() {var implementors = {
"massa_async_pool":[["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_async_pool/struct.AsyncPoolChanges.html\" title=\"struct massa_async_pool::AsyncPoolChanges\">AsyncPoolChanges</a>&gt; for <a class=\"struct\" href=\"massa_async_pool/struct.AsyncPoolChangesSerializer.html\" title=\"struct massa_async_pool::AsyncPoolChangesSerializer\">AsyncPoolChangesSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;(<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/cmp/struct.Reverse.html\" title=\"struct core::cmp::Reverse\">Reverse</a>&lt;<a class=\"struct\" href=\"https://docs.rs/num-rational/0.4/num_rational/struct.Ratio.html\" title=\"struct num_rational::Ratio\">Ratio</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt;&gt;, Slot, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>)&gt; for <a class=\"struct\" href=\"massa_async_pool/struct.AsyncMessageIdSerializer.html\" title=\"struct massa_async_pool::AsyncMessageIdSerializer\">AsyncMessageIdSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_async_pool/struct.AsyncMessage.html\" title=\"struct massa_async_pool::AsyncMessage\">AsyncMessage</a>&gt; for <a class=\"struct\" href=\"massa_async_pool/struct.AsyncMessageSerializer.html\" title=\"struct massa_async_pool::AsyncMessageSerializer\">AsyncMessageSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;(<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/cmp/struct.Reverse.html\" title=\"struct core::cmp::Reverse\">Reverse</a>&lt;<a class=\"struct\" href=\"https://docs.rs/num-rational/0.4/num_rational/struct.Ratio.html\" title=\"struct num_rational::Ratio\">Ratio</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt;&gt;, Slot, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>), <a class=\"struct\" href=\"massa_async_pool/struct.AsyncMessage.html\" title=\"struct massa_async_pool::AsyncMessage\">AsyncMessage</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_async_pool/struct.AsyncPoolSerializer.html\" title=\"struct massa_async_pool::AsyncPoolSerializer\">AsyncPoolSerializer</a>"]],
"massa_bootstrap":[["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"enum\" href=\"massa_bootstrap/enum.BootstrapServerMessage.html\" title=\"enum massa_bootstrap::BootstrapServerMessage\">BootstrapServerMessage</a>&gt; for <a class=\"struct\" href=\"massa_bootstrap/struct.BootstrapServerMessageSerializer.html\" title=\"struct massa_bootstrap::BootstrapServerMessageSerializer\">BootstrapServerMessageSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"enum\" href=\"massa_bootstrap/enum.BootstrapClientMessage.html\" title=\"enum massa_bootstrap::BootstrapClientMessage\">BootstrapClientMessage</a>&gt; for <a class=\"struct\" href=\"massa_bootstrap/struct.BootstrapClientMessageSerializer.html\" title=\"struct massa_bootstrap::BootstrapClientMessageSerializer\">BootstrapClientMessageSerializer</a>"]],
"massa_consensus_exports":[["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_consensus_exports/bootstrapable_graph/struct.BootstrapableGraph.html\" title=\"struct massa_consensus_exports::bootstrapable_graph::BootstrapableGraph\">BootstrapableGraph</a>&gt; for <a class=\"struct\" href=\"massa_consensus_exports/bootstrapable_graph/struct.BootstrapableGraphSerializer.html\" title=\"struct massa_consensus_exports::bootstrapable_graph::BootstrapableGraphSerializer\">BootstrapableGraphSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_consensus_exports/export_active_block/struct.ExportActiveBlock.html\" title=\"struct massa_consensus_exports::export_active_block::ExportActiveBlock\">ExportActiveBlock</a>&gt; for <a class=\"struct\" href=\"massa_consensus_exports/export_active_block/struct.ExportActiveBlockSerializer.html\" title=\"struct massa_consensus_exports::export_active_block::ExportActiveBlockSerializer\">ExportActiveBlockSerializer</a>"]],
"massa_executed_ops":[["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;Slot, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html\" title=\"struct std::collections::hash::set::HashSet\">HashSet</a>&lt;OperationId, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;HashMapper&lt;OperationId&gt;&gt;&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_executed_ops/struct.ExecutedOpsSerializer.html\" title=\"struct massa_executed_ops::ExecutedOpsSerializer\">ExecutedOpsSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html\" title=\"struct std::collections::hash::map::HashMap\">HashMap</a>&lt;OperationId, (<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>, Slot), <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;HashMapper&lt;OperationId&gt;&gt;&gt;&gt; for <a class=\"struct\" href=\"massa_executed_ops/struct.ExecutedOpsChangesSerializer.html\" title=\"struct massa_executed_ops::ExecutedOpsChangesSerializer\">ExecutedOpsChangesSerializer</a>"]],
"massa_final_state":[["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_final_state/struct.StateChanges.html\" title=\"struct massa_final_state::StateChanges\">StateChanges</a>&gt; for <a class=\"struct\" href=\"massa_final_state/struct.StateChangesSerializer.html\" title=\"struct massa_final_state::StateChangesSerializer\">StateChangesSerializer</a>"]],
"massa_hash":[["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_hash/struct.Hash.html\" title=\"struct massa_hash::Hash\">Hash</a>&gt; for <a class=\"struct\" href=\"massa_hash/struct.HashSerializer.html\" title=\"struct massa_hash::HashSerializer\">HashSerializer</a>"]],
"massa_ledger_exports":[["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_ledger_exports/struct.Key.html\" title=\"struct massa_ledger_exports::Key\">Key</a>&gt; for <a class=\"struct\" href=\"massa_ledger_exports/struct.KeySerializer.html\" title=\"struct massa_ledger_exports::KeySerializer\">KeySerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;, <a class=\"enum\" href=\"massa_ledger_exports/enum.SetOrDelete.html\" title=\"enum massa_ledger_exports::SetOrDelete\">SetOrDelete</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_ledger_exports/struct.DatastoreUpdateSerializer.html\" title=\"struct massa_ledger_exports::DatastoreUpdateSerializer\">DatastoreUpdateSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_ledger_exports/struct.LedgerEntryUpdate.html\" title=\"struct massa_ledger_exports::LedgerEntryUpdate\">LedgerEntryUpdate</a>&gt; for <a class=\"struct\" href=\"massa_ledger_exports/struct.LedgerEntryUpdateSerializer.html\" title=\"struct massa_ledger_exports::LedgerEntryUpdateSerializer\">LedgerEntryUpdateSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_ledger_exports/struct.LedgerChanges.html\" title=\"struct massa_ledger_exports::LedgerChanges\">LedgerChanges</a>&gt; for <a class=\"struct\" href=\"massa_ledger_exports/struct.LedgerChangesSerializer.html\" title=\"struct massa_ledger_exports::LedgerChangesSerializer\">LedgerChangesSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_ledger_exports/struct.LedgerEntry.html\" title=\"struct massa_ledger_exports::LedgerEntry\">LedgerEntry</a>&gt; for <a class=\"struct\" href=\"massa_ledger_exports/struct.LedgerEntrySerializer.html\" title=\"struct massa_ledger_exports::LedgerEntrySerializer\">LedgerEntrySerializer</a>"]],
"massa_models":[["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"enum\" href=\"massa_models/address/enum.Address.html\" title=\"enum massa_models::address::Address\">Address</a>&gt; for <a class=\"struct\" href=\"massa_models/address/struct.AddressSerializer.html\" title=\"struct massa_models::address::AddressSerializer\">AddressSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/amount/struct.Amount.html\" title=\"struct massa_models::amount::Amount\">Amount</a>&gt; for <a class=\"struct\" href=\"massa_models/amount/struct.AmountSerializer.html\" title=\"struct massa_models::amount::AmountSerializer\">AmountSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/block/struct.Block.html\" title=\"struct massa_models::block::Block\">Block</a>&gt; for <a class=\"struct\" href=\"massa_models/block/struct.BlockSerializer.html\" title=\"struct massa_models::block::BlockSerializer\">BlockSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/block_header/struct.BlockHeader.html\" title=\"struct massa_models::block_header::BlockHeader\">BlockHeader</a>&gt; for <a class=\"struct\" href=\"massa_models/block_header/struct.BlockHeaderSerializer.html\" title=\"struct massa_models::block_header::BlockHeaderSerializer\">BlockHeaderSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/block_id/struct.BlockId.html\" title=\"struct massa_models::block_id::BlockId\">BlockId</a>&gt; for <a class=\"struct\" href=\"massa_models/block_id/struct.BlockIdSerializer.html\" title=\"struct massa_models::block_id::BlockIdSerializer\">BlockIdSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/clique/struct.Clique.html\" title=\"struct massa_models::clique::Clique\">Clique</a>&gt; for <a class=\"struct\" href=\"massa_models/clique/struct.CliqueSerializer.html\" title=\"struct massa_models::clique::CliqueSerializer\">CliqueSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_models/datastore/struct.DatastoreSerializer.html\" title=\"struct massa_models::datastore::DatastoreSerializer\">DatastoreSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/endorsement/struct.Endorsement.html\" title=\"struct massa_models::endorsement::Endorsement\">Endorsement</a>&gt; for <a class=\"struct\" href=\"massa_models/endorsement/struct.EndorsementSerializer.html\" title=\"struct massa_models::endorsement::EndorsementSerializer\">EndorsementSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/endorsement/struct.Endorsement.html\" title=\"struct massa_models::endorsement::Endorsement\">Endorsement</a>&gt; for <a class=\"struct\" href=\"massa_models/endorsement/struct.EndorsementSerializerLW.html\" title=\"struct massa_models::endorsement::EndorsementSerializerLW\">EndorsementSerializerLW</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/ledger/struct.LedgerData.html\" title=\"struct massa_models::ledger::LedgerData\">LedgerData</a>&gt; for <a class=\"struct\" href=\"massa_models/ledger/struct.LedgerDataSerializer.html\" title=\"struct massa_models::ledger::LedgerDataSerializer\">LedgerDataSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/ledger/struct.LedgerChange.html\" title=\"struct massa_models::ledger::LedgerChange\">LedgerChange</a>&gt; for <a class=\"struct\" href=\"massa_models/ledger/struct.LedgerChangeSerializer.html\" title=\"struct massa_models::ledger::LedgerChangeSerializer\">LedgerChangeSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/ledger/struct.LedgerChanges.html\" title=\"struct massa_models::ledger::LedgerChanges\">LedgerChanges</a>&gt; for <a class=\"struct\" href=\"massa_models/ledger/struct.LedgerChangesSerializer.html\" title=\"struct massa_models::ledger::LedgerChangesSerializer\">LedgerChangesSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/operation/struct.OperationId.html\" title=\"struct massa_models::operation::OperationId\">OperationId</a>&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationIdSerializer.html\" title=\"struct massa_models::operation::OperationIdSerializer\">OperationIdSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/operation/struct.Operation.html\" title=\"struct massa_models::operation::Operation\">Operation</a>&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationSerializer.html\" title=\"struct massa_models::operation::OperationSerializer\">OperationSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"enum\" href=\"massa_models/operation/enum.OperationType.html\" title=\"enum massa_models::operation::OperationType\">OperationType</a>&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationTypeSerializer.html\" title=\"struct massa_models::operation::OperationTypeSerializer\">OperationTypeSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"massa_models/operation/struct.OperationId.html\" title=\"struct massa_models::operation::OperationId\">OperationId</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationIdsSerializer.html\" title=\"struct massa_models::operation::OperationIdsSerializer\">OperationIdsSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html\" title=\"struct std::collections::hash::set::HashSet\">HashSet</a>&lt;<a class=\"struct\" href=\"massa_models/operation/struct.OperationPrefixId.html\" title=\"struct massa_models::operation::OperationPrefixId\">OperationPrefixId</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;<a class=\"struct\" href=\"massa_models/prehash/struct.HashMapper.html\" title=\"struct massa_models::prehash::HashMapper\">HashMapper</a>&lt;<a class=\"struct\" href=\"massa_models/operation/struct.OperationPrefixId.html\" title=\"struct massa_models::operation::OperationPrefixId\">OperationPrefixId</a>&gt;&gt;&gt;&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationPrefixIdsSerializer.html\" title=\"struct massa_models::operation::OperationPrefixIdsSerializer\">OperationPrefixIdsSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"massa_models/secure_share/struct.SecureShare.html\" title=\"struct massa_models::secure_share::SecureShare\">SecureShare</a>&lt;<a class=\"struct\" href=\"massa_models/operation/struct.Operation.html\" title=\"struct massa_models::operation::Operation\">Operation</a>, <a class=\"struct\" href=\"massa_models/operation/struct.OperationId.html\" title=\"struct massa_models::operation::OperationId\">OperationId</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_models/operation/struct.OperationsSerializer.html\" title=\"struct massa_models::operation::OperationsSerializer\">OperationsSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/rolls/struct.RollUpdate.html\" title=\"struct massa_models::rolls::RollUpdate\">RollUpdate</a>&gt; for <a class=\"struct\" href=\"massa_models/rolls/struct.RollUpdateSerializer.html\" title=\"struct massa_models::rolls::RollUpdateSerializer\">RollUpdateSerializer</a>"],["impl&lt;T, ID&gt; <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/secure_share/struct.SecureShare.html\" title=\"struct massa_models::secure_share::SecureShare\">SecureShare</a>&lt;T, ID&gt;&gt; for <a class=\"struct\" href=\"massa_models/secure_share/struct.SecureShareSerializer.html\" title=\"struct massa_models::secure_share::SecureShareSerializer\">SecureShareSerializer</a><span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"massa_models/secure_share/trait.SecureShareContent.html\" title=\"trait massa_models::secure_share::SecureShareContent\">SecureShareContent</a>,\n    ID: <a class=\"trait\" href=\"massa_models/secure_share/trait.Id.html\" title=\"trait massa_models::secure_share::Id\">Id</a>,</span>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/std/net/ip_addr/enum.IpAddr.html\" title=\"enum std::net::ip_addr::IpAddr\">IpAddr</a>&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.IpAddrSerializer.html\" title=\"struct massa_models::serialization::IpAddrSerializer\">IpAddrSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.VecU8Serializer.html\" title=\"struct massa_models::serialization::VecU8Serializer\">VecU8Serializer</a>"],["impl&lt;T, ST&gt; <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;T, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.VecSerializer.html\" title=\"struct massa_models::serialization::VecSerializer\">VecSerializer</a>&lt;T, ST&gt;<span class=\"where fmt-newline\">where\n    ST: <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;T&gt;,</span>"],["impl&lt;T, ST&gt; <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html\" title=\"struct std::collections::hash::set::HashSet\">HashSet</a>&lt;T, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;<a class=\"struct\" href=\"massa_models/prehash/struct.HashMapper.html\" title=\"struct massa_models::prehash::HashMapper\">HashMapper</a>&lt;T&gt;&gt;&gt;&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.PreHashSetSerializer.html\" title=\"struct massa_models::serialization::PreHashSetSerializer\">PreHashSetSerializer</a>&lt;T, ST&gt;<span class=\"where fmt-newline\">where\n    ST: <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;T&gt;,\n    T: <a class=\"trait\" href=\"massa_models/prehash/trait.PreHashed.html\" title=\"trait massa_models::prehash::PreHashed\">PreHashed</a>,</span>"],["impl&lt;SL, L&gt; <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.StringSerializer.html\" title=\"struct massa_models::serialization::StringSerializer\">StringSerializer</a>&lt;SL, L&gt;<span class=\"where fmt-newline\">where\n    SL: <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;L&gt;,\n    L: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt;,</span>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;BitVec&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, Lsb0&gt;&gt; for <a class=\"struct\" href=\"massa_models/serialization/struct.BitVecSerializer.html\" title=\"struct massa_models::serialization::BitVecSerializer\">BitVecSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/slot/struct.Slot.html\" title=\"struct massa_models::slot::Slot\">Slot</a>&gt; for <a class=\"struct\" href=\"massa_models/slot/struct.SlotSerializer.html\" title=\"struct massa_models::slot::SlotSerializer\">SlotSerializer</a>"],["impl&lt;T, ST&gt; <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"enum\" href=\"massa_models/streaming_step/enum.StreamingStep.html\" title=\"enum massa_models::streaming_step::StreamingStep\">StreamingStep</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"massa_models/streaming_step/struct.StreamingStepSerializer.html\" title=\"struct massa_models::streaming_step::StreamingStepSerializer\">StreamingStepSerializer</a>&lt;T, ST&gt;<span class=\"where fmt-newline\">where\n    ST: <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;T&gt;,</span>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_models/version/struct.Version.html\" title=\"struct massa_models::version::Version\">Version</a>&gt; for <a class=\"struct\" href=\"massa_models/version/struct.VersionSerializer.html\" title=\"struct massa_models::version::VersionSerializer\">VersionSerializer</a>"]],
"massa_network_exports":[["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_network_exports/struct.BootstrapPeers.html\" title=\"struct massa_network_exports::BootstrapPeers\">BootstrapPeers</a>&gt; for <a class=\"struct\" href=\"massa_network_exports/struct.BootstrapPeersSerializer.html\" title=\"struct massa_network_exports::BootstrapPeersSerializer\">BootstrapPeersSerializer</a>"]],
"massa_pos_exports":[["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_pos_exports/struct.CycleInfo.html\" title=\"struct massa_pos_exports::CycleInfo\">CycleInfo</a>&gt; for <a class=\"struct\" href=\"massa_pos_exports/struct.CycleInfoSerializer.html\" title=\"struct massa_pos_exports::CycleInfoSerializer\">CycleInfoSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html\" title=\"struct std::collections::hash::map::HashMap\">HashMap</a>&lt;Address, <a class=\"struct\" href=\"massa_pos_exports/struct.ProductionStats.html\" title=\"struct massa_pos_exports::ProductionStats\">ProductionStats</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;HashMapper&lt;Address&gt;&gt;&gt;&gt; for <a class=\"struct\" href=\"massa_pos_exports/struct.ProductionStatsSerializer.html\" title=\"struct massa_pos_exports::ProductionStatsSerializer\">ProductionStatsSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_pos_exports/struct.DeferredCredits.html\" title=\"struct massa_pos_exports::DeferredCredits\">DeferredCredits</a>&gt; for <a class=\"struct\" href=\"massa_pos_exports/struct.DeferredCreditsSerializer.html\" title=\"struct massa_pos_exports::DeferredCreditsSerializer\">DeferredCreditsSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html\" title=\"struct std::collections::hash::map::HashMap\">HashMap</a>&lt;Address, Amount, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;HashMapper&lt;Address&gt;&gt;&gt;&gt; for <a class=\"struct\" href=\"massa_pos_exports/struct.CreditsSerializer.html\" title=\"struct massa_pos_exports::CreditsSerializer\">CreditsSerializer</a>"],["impl <a class=\"trait\" href=\"massa_serialization/trait.Serializer.html\" title=\"trait massa_serialization::Serializer\">Serializer</a>&lt;<a class=\"struct\" href=\"massa_pos_exports/struct.PoSChanges.html\" title=\"struct massa_pos_exports::PoSChanges\">PoSChanges</a>&gt; for <a class=\"struct\" href=\"massa_pos_exports/struct.PoSChangesSerializer.html\" title=\"struct massa_pos_exports::PoSChangesSerializer\">PoSChangesSerializer</a>"]],
"massa_serialization":[],
"massa_time":[["impl Serializer&lt;<a class=\"struct\" href=\"massa_time/struct.MassaTime.html\" title=\"struct massa_time::MassaTime\">MassaTime</a>&gt; for <a class=\"struct\" href=\"massa_time/struct.MassaTimeSerializer.html\" title=\"struct massa_time::MassaTimeSerializer\">MassaTimeSerializer</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()