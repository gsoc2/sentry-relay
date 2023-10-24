(function() {var implementors = {
"relay_base_schema":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"relay_base_schema/events/enum.EventType.html\" title=\"enum relay_base_schema::events::EventType\">EventType</a>&gt; for <a class=\"enum\" href=\"relay_base_schema/data_category/enum.DataCategory.html\" title=\"enum relay_base_schema::data_category::DataCategory\">DataCategory</a>"]],
"relay_cabi":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"relay_cabi/struct.RelayStr.html\" title=\"struct relay_cabi::RelayStr\">RelayStr</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/uuid/1.3.0/uuid/struct.Uuid.html\" title=\"struct uuid::Uuid\">Uuid</a>&gt; for <a class=\"struct\" href=\"relay_cabi/struct.RelayUuid.html\" title=\"struct relay_cabi::RelayUuid\">RelayUuid</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"relay_cabi/struct.RelayStr.html\" title=\"struct relay_cabi::RelayStr\">RelayStr</a>"]],
"relay_common":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"relay_common/glob2/struct.Glob.html\" title=\"struct relay_common::glob2::Glob\">Glob</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"relay_common/glob2/struct.Glob.html\" title=\"struct relay_common::glob2::Glob\">Glob</a>"]],
"relay_config":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.u32.html\">u32</a>&gt; for <a class=\"struct\" href=\"relay_config/struct.ByteSize.html\" title=\"struct relay_config::ByteSize\">ByteSize</a>"]],
"relay_event_normalization":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;<a class=\"struct\" href=\"relay_event_normalization/struct.TransactionNameRule.html\" title=\"struct relay_event_normalization::TransactionNameRule\">TransactionNameRule</a>&gt; for <a class=\"struct\" href=\"relay_event_normalization/struct.SpanDescriptionRule.html\" title=\"struct relay_event_normalization::SpanDescriptionRule\">SpanDescriptionRule</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/serde_json/1.0.93/serde_json/error/struct.Error.html\" title=\"struct serde_json::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"relay_event_normalization/replay/enum.ReplayError.html\" title=\"enum relay_event_normalization::replay::ReplayError\">ReplayError</a>"]],
"relay_event_schema":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Message.html\" title=\"struct relay_event_schema::protocol::Message\">Message</a>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.DebugId.html\" title=\"struct relay_event_schema::protocol::DebugId\">DebugId</a><span class=\"where fmt-newline\">where\n    DebugId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt;,</span>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;Annotated&lt;T&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.PairList.html\" title=\"struct relay_event_schema::protocol::PairList\">PairList</a>&lt;T&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Fingerprint.html\" title=\"struct relay_event_schema::protocol::Fingerprint\">Fingerprint</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, Annotated&lt;Value&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.MonitorContext.html\" title=\"struct relay_event_schema::protocol::MonitorContext\">MonitorContext</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.JsonLenientString.html\" title=\"struct relay_event_schema::protocol::JsonLenientString\">JsonLenientString</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.HeaderName.html\" title=\"struct relay_event_schema::protocol::HeaderName\">HeaderName</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.HeaderName.html\" title=\"struct relay_event_schema::protocol::HeaderName\">HeaderName</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.HeaderValue.html\" title=\"struct relay_event_schema::protocol::HeaderValue\">HeaderValue</a>"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;Value&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.ExtraValue.html\" title=\"struct relay_event_schema::protocol::ExtraValue\">ExtraValue</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"relay_event_schema/protocol/enum.Context.html\" title=\"enum relay_event_schema::protocol::Context\">Context</a>&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.ContextInner.html\" title=\"struct relay_event_schema::protocol::ContextInner\">ContextInner</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"relay_event_schema/protocol/struct.RawStacktrace.html\" title=\"struct relay_event_schema::protocol::RawStacktrace\">RawStacktrace</a>&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Stacktrace.html\" title=\"struct relay_event_schema::protocol::Stacktrace\">Stacktrace</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/chrono/latest/chrono/datetime/struct.DateTime.html\" title=\"struct chrono::datetime::DateTime\">DateTime</a>&lt;<a class=\"struct\" href=\"https://docs.rs/chrono/latest/chrono/offset/utc/struct.Utc.html\" title=\"struct chrono::offset::utc::Utc\">Utc</a>&gt;&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Timestamp.html\" title=\"struct relay_event_schema::protocol::Timestamp\">Timestamp</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, Annotated&lt;Value&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.FrameVars.html\" title=\"struct relay_event_schema::protocol::FrameVars\">FrameVars</a>"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.NativeImagePath.html\" title=\"struct relay_event_schema::protocol::NativeImagePath\">NativeImagePath</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.73.0/core/net/ip_addr/enum.IpAddr.html\" title=\"enum core::net::ip_addr::IpAddr\">IpAddr</a>&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.IpAddr.html\" title=\"struct relay_event_schema::protocol::IpAddr\">IpAddr</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;<a class=\"struct\" href=\"relay_event_schema/protocol/struct.Event.html\" title=\"struct relay_event_schema::protocol::Event\">Event</a>&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.Span.html\" title=\"struct relay_event_schema::protocol::Span\">Span</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.HeaderValue.html\" title=\"struct relay_event_schema::protocol::HeaderValue\">HeaderValue</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.LenientString.html\" title=\"struct relay_event_schema::protocol::LenientString\">LenientString</a>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.CodeId.html\" title=\"struct relay_event_schema::protocol::CodeId\">CodeId</a><span class=\"where fmt-newline\">where\n    CodeId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt;,</span>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.LogEntry.html\" title=\"struct relay_event_schema::protocol::LogEntry\">LogEntry</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"relay_event_schema/protocol/struct.Stacktrace.html\" title=\"struct relay_event_schema::protocol::Stacktrace\">Stacktrace</a>&gt; for <a class=\"struct\" href=\"relay_event_schema/protocol/struct.RawStacktrace.html\" title=\"struct relay_event_schema::protocol::RawStacktrace\">RawStacktrace</a>"]],
"relay_filter":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"relay_filter/csp/struct.SchemeDomainPort.html\" title=\"struct relay_filter::csp::SchemeDomainPort\">SchemeDomainPort</a>"]],
"relay_kafka":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"enum\" href=\"relay_kafka/enum.TopicAssignment.html\" title=\"enum relay_kafka::TopicAssignment\">TopicAssignment</a>"]],
"relay_monitors":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/serde_json/1.0.93/serde_json/error/struct.Error.html\" title=\"struct serde_json::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"relay_monitors/enum.ProcessCheckInError.html\" title=\"enum relay_monitors::ProcessCheckInError\">ProcessCheckInError</a>"]],
"relay_pii":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/core/str/error/struct.Utf8Error.html\" title=\"struct core::str::error::Utf8Error\">Utf8Error</a>&gt; for <a class=\"enum\" href=\"relay_pii/enum.ScrubMinidumpError.html\" title=\"enum relay_pii::ScrubMinidumpError\">ScrubMinidumpError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"relay_pii/enum.ScrubMinidumpError.html\" title=\"enum relay_pii::ScrubMinidumpError\">ScrubMinidumpError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/core/num/error/struct.TryFromIntError.html\" title=\"struct core::num::error::TryFromIntError\">TryFromIntError</a>&gt; for <a class=\"enum\" href=\"relay_pii/enum.ScrubMinidumpError.html\" title=\"enum relay_pii::ScrubMinidumpError\">ScrubMinidumpError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"relay_pii/struct.LazyPattern.html\" title=\"struct relay_pii::LazyPattern\">LazyPattern</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;ValueType&gt; for <a class=\"enum\" href=\"relay_pii/enum.SelectorSpec.html\" title=\"enum relay_pii::SelectorSpec\">SelectorSpec</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Utf16Error&gt; for <a class=\"enum\" href=\"relay_pii/enum.ScrubMinidumpError.html\" title=\"enum relay_pii::ScrubMinidumpError\">ScrubMinidumpError</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"relay_pii/struct.ReplaceRedaction.html\" title=\"struct relay_pii::ReplaceRedaction\">ReplaceRedaction</a>"]],
"relay_profiling":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/serde_json/1.0.93/serde_json/error/struct.Error.html\" title=\"struct serde_json::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"relay_profiling/enum.ProfileError.html\" title=\"enum relay_profiling::ProfileError\">ProfileError</a>"]],
"relay_protocol":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"relay_protocol/struct.Annotated.html\" title=\"struct relay_protocol::Annotated\">Annotated</a>&lt;<a class=\"enum\" href=\"relay_protocol/enum.Value.html\" title=\"enum relay_protocol::Value\">Value</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Value.html\" title=\"enum relay_protocol::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.ErrorKind.html\" title=\"enum relay_protocol::ErrorKind\">ErrorKind</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"enum\" href=\"relay_protocol/enum.Value.html\" title=\"enum relay_protocol::Value\">Value</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Val.html\" title=\"enum relay_protocol::Val\">Val</a>&lt;'a&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.u64.html\">u64</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Value.html\" title=\"enum relay_protocol::Value\">Value</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.str.html\">str</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.ErrorKind.html\" title=\"enum relay_protocol::ErrorKind\">ErrorKind</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.str.html\">str</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Val.html\" title=\"enum relay_protocol::Val\">Val</a>&lt;'a&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"struct\" href=\"relay_protocol/struct.Annotated.html\" title=\"struct relay_protocol::Annotated\">Annotated</a>&lt;<a class=\"enum\" href=\"relay_protocol/enum.Value.html\" title=\"enum relay_protocol::Value\">Value</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Value.html\" title=\"enum relay_protocol::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"relay_protocol/enum.ErrorKind.html\" title=\"enum relay_protocol::ErrorKind\">ErrorKind</a>&gt; for <a class=\"struct\" href=\"relay_protocol/struct.Error.html\" title=\"struct relay_protocol::Error\">Error</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"relay_protocol/enum.Value.html\" title=\"enum relay_protocol::Value\">Value</a>&gt; for <a class=\"enum\" href=\"https://docs.rs/serde_json/1.0.93/serde_json/value/enum.Value.html\" title=\"enum serde_json::value::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.bool.html\">bool</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Val.html\" title=\"enum relay_protocol::Val\">Val</a>&lt;'_&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://docs.rs/serde_json/1.0.93/serde_json/value/enum.Value.html\" title=\"enum serde_json::value::Value\">Value</a>&gt; for <a class=\"struct\" href=\"relay_protocol/struct.Annotated.html\" title=\"struct relay_protocol::Annotated\">Annotated</a>&lt;<a class=\"enum\" href=\"relay_protocol/enum.Value.html\" title=\"enum relay_protocol::Value\">Value</a>&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"relay_protocol/struct.Annotated.html\" title=\"struct relay_protocol::Annotated\">Annotated</a>&lt;<a class=\"enum\" href=\"relay_protocol/enum.Value.html\" title=\"enum relay_protocol::Value\">Value</a>&gt;&gt; for <a class=\"enum\" href=\"https://docs.rs/serde_json/1.0.93/serde_json/value/enum.Value.html\" title=\"enum serde_json::value::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.f64.html\">f64</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Value.html\" title=\"enum relay_protocol::Value\">Value</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.str.html\">str</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Value.html\" title=\"enum relay_protocol::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.bool.html\">bool</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Value.html\" title=\"enum relay_protocol::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/uuid/1.3.0/uuid/struct.Uuid.html\" title=\"struct uuid::Uuid\">Uuid</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Val.html\" title=\"enum relay_protocol::Val\">Val</a>&lt;'_&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.f64.html\">f64</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Val.html\" title=\"enum relay_protocol::Val\">Val</a>&lt;'_&gt;"],["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.reference.html\">&amp;'a T</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Val.html\" title=\"enum relay_protocol::Val\">Val</a>&lt;'a&gt;<span class=\"where fmt-newline\">where\n    <a class=\"enum\" href=\"relay_protocol/enum.Val.html\" title=\"enum relay_protocol::Val\">Val</a>&lt;'a&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt;,\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,</span>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.i64.html\">i64</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Value.html\" title=\"enum relay_protocol::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.73.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Value.html\" title=\"enum relay_protocol::Value\">Value</a>"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"relay_protocol/struct.Annotated.html\" title=\"struct relay_protocol::Annotated\">Annotated</a>&lt;T&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.u64.html\">u64</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Val.html\" title=\"enum relay_protocol::Val\">Val</a>&lt;'_&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.73.0/std/primitive.i64.html\">i64</a>&gt; for <a class=\"enum\" href=\"relay_protocol/enum.Val.html\" title=\"enum relay_protocol::Val\">Val</a>&lt;'_&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.73.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"relay_protocol/struct.Annotated.html\" title=\"struct relay_protocol::Annotated\">Annotated</a>&lt;T&gt;"]],
"relay_replays":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/serde_json/1.0.93/serde_json/error/struct.Error.html\" title=\"struct serde_json::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"relay_replays/recording/enum.ParseRecordingError.html\" title=\"enum relay_replays::recording::ParseRecordingError\">ParseRecordingError</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()