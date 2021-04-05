/// source code position
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourcePosition {
    /// line number
    #[prost(int32, tag="1")]
    pub row: i32,
    /// char offset
    #[prost(int32, tag="2")]
    pub col: i32,
    /// length, 0 if unknown
    #[prost(int32, tag="3")]
    pub len: i32,
}
/// syntax error in source code
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyntaxError {
    /// kind of this error
    #[prost(enumeration="SyntaxErrorKind", tag="1")]
    pub kind: i32,
    /// error message 
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    /// source code location
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<SourcePosition>,
    /// contra location 
    #[prost(message, optional, tag="4")]
    pub contra_position: ::core::option::Option<SourcePosition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompiliationError {
    /// error type
    #[prost(enumeration="CompiliationErrorKind", tag="1")]
    pub kind: i32,
    /// error message
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    /// source code location
    #[prost(message, optional, tag="3")]
    pub position: ::core::option::Option<SourcePosition>,
    /// contra location 
    #[prost(message, optional, tag="4")]
    pub contra_position: ::core::option::Option<SourcePosition>,
}
/// Result of normalization
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizeResult {
    /// Fatal error encountered in compiliation
    #[prost(message, optional, tag="1")]
    pub compiliation_error: ::core::option::Option<CompiliationError>,
    /// Syntax error occured in compiliation
    #[prost(message, repeated, tag="2")]
    pub syntax_errors: ::prost::alloc::vec::Vec<SyntaxError>,
    /// When there is no fatal errors, this is the root par
    #[prost(message, optional, tag="4")]
    pub par: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionError {
    /// error type
    #[prost(enumeration="ExecutionErrorKind", tag="1")]
    pub kind: i32,
    /// error message
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
///*
/// Rholang process
///
/// For example, `@0!(1) | @2!(3) | for(x <- @0) { Nil }` has two sends
/// and one receive.
///
/// The Nil process is a `Par` with no sends, receives, etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Par {
    #[prost(message, repeated, tag="1")]
    pub sends: ::prost::alloc::vec::Vec<Send>,
    #[prost(message, repeated, tag="2")]
    pub receives: ::prost::alloc::vec::Vec<Receive>,
    #[prost(message, repeated, tag="4")]
    pub news: ::prost::alloc::vec::Vec<New>,
    #[prost(message, repeated, tag="5")]
    pub exprs: ::prost::alloc::vec::Vec<Expr>,
    #[prost(message, repeated, tag="6")]
    pub matches: ::prost::alloc::vec::Vec<Match>,
    /// unforgeable names
    #[prost(message, repeated, tag="7")]
    pub unforgeables: ::prost::alloc::vec::Vec<GUnforgeable>,
    #[prost(message, repeated, tag="11")]
    pub bundles: ::prost::alloc::vec::Vec<Bundle>,
    #[prost(message, repeated, tag="8")]
    pub connectives: ::prost::alloc::vec::Vec<Connective>,
    ///[(scalapb.field).type = "coop.rchain.models.AlwaysEqual[scala.collection.immutable.BitSet]"];
    #[prost(message, optional, tag="9")]
    pub locally_free: ::core::option::Option<super::bitset::BitSet>,
    #[prost(bool, tag="10")]
    pub connective_used: bool,
}
///*
/// Either rholang code or code built in to the interpreter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaggedContinuation {
    #[prost(oneof="tagged_continuation::TaggedCont", tags="1, 2")]
    pub tagged_cont: ::core::option::Option<tagged_continuation::TaggedCont>,
}
/// Nested message and enum types in `TaggedContinuation`.
pub mod tagged_continuation {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TaggedCont {
        #[prost(message, tag="1")]
        ParBody(super::ParWithRandom),
        #[prost(int64, tag="2")]
        ScalaBodyRef(i64),
    }
}
///*
/// Rholang code along with the state of a split random number
/// generator for generating new unforgeable names.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParWithRandom {
    #[prost(message, optional, tag="1")]
    pub body: ::core::option::Option<Par>,
    ///[(scalapb.field).type = "coop.rchain.crypto.hash.Blake2b512Random"];
    #[prost(bytes="vec", tag="2")]
    pub random_state: ::prost::alloc::vec::Vec<u8>,
}
///*
/// Cost of the performed operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PCost {
    #[prost(uint64, tag="1")]
    pub cost: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListParWithRandom {
    #[prost(message, repeated, tag="1")]
    pub pars: ::prost::alloc::vec::Vec<Par>,
    ///[(scalapb.field).type = "coop.rchain.crypto.hash.Blake2b512Random"];
    #[prost(bytes="vec", tag="2")]
    pub random_state: ::prost::alloc::vec::Vec<u8>,
}
/// While we use vars in both positions, when producing the normalized
/// representation we need a discipline to track whether a var is a name or a
/// process.
/// These are DeBruijn levels
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Var {
    #[prost(oneof="var::VarInstance", tags="1, 2, 3")]
    pub var_instance: ::core::option::Option<var::VarInstance>,
}
/// Nested message and enum types in `Var`.
pub mod var {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WildcardMsg {
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VarInstance {
        #[prost(sint32, tag="1")]
        BoundVar(i32),
        #[prost(sint32, tag="2")]
        FreeVar(i32),
        #[prost(message, tag="3")]
        Wildcard(WildcardMsg),
    }
}
///*
/// Nothing can be received from a (quoted) bundle with `readFlag = false`.
/// Likeise nothing can be sent to a (quoted) bundle with `writeFlag = false`.
///
/// If both flags are set to false, bundle allows only for equivalance check.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bundle {
    #[prost(message, optional, tag="1")]
    pub body: ::core::option::Option<Par>,
    /// flag indicating whether bundle is writeable
    #[prost(bool, tag="2")]
    pub write_flag: bool,
    /// flag indicating whether bundle is readable
    #[prost(bool, tag="3")]
    pub read_flag: bool,
}
///*
/// A send is written `chan!(data)` or `chan!!(data)` for a persistent send.
///
/// Upon send, all free variables in data are substituted with their values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Send {
    #[prost(message, optional, tag="1")]
    pub chan: ::core::option::Option<Par>,
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<Par>,
    #[prost(bool, tag="3")]
    pub persistent: bool,
    ///[(scalapb.field).type = "coop.rchain.models.AlwaysEqual[scala.collection.immutable.BitSet]"];
    #[prost(message, optional, tag="5")]
    pub locally_free: ::core::option::Option<super::bitset::BitSet>,
    #[prost(bool, tag="6")]
    pub connective_used: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveBind {
    #[prost(message, repeated, tag="1")]
    pub patterns: ::prost::alloc::vec::Vec<Par>,
    #[prost(message, optional, tag="2")]
    pub source: ::core::option::Option<Par>,
    #[prost(message, optional, tag="3")]
    pub remainder: ::core::option::Option<Var>,
    #[prost(int32, tag="4")]
    pub free_count: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindPattern {
    #[prost(message, repeated, tag="1")]
    pub patterns: ::prost::alloc::vec::Vec<Par>,
    #[prost(message, optional, tag="2")]
    pub remainder: ::core::option::Option<Var>,
    #[prost(int32, tag="3")]
    pub free_count: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBindPatterns {
    #[prost(message, repeated, tag="1")]
    pub patterns: ::prost::alloc::vec::Vec<BindPattern>,
}
///*
/// A receive is written `for(binds) { body }`
/// i.e. `for(patterns <- source) { body }`
/// or for a persistent recieve: `for(patterns <= source) { body }`.
///
/// It's an error for free Variable to occur more than once in a pattern.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Receive {
    #[prost(message, repeated, tag="1")]
    pub binds: ::prost::alloc::vec::Vec<ReceiveBind>,
    #[prost(message, optional, tag="2")]
    pub body: ::core::option::Option<Par>,
    #[prost(bool, tag="3")]
    pub persistent: bool,
    #[prost(bool, tag="4")]
    pub peek: bool,
    #[prost(int32, tag="5")]
    pub bind_count: i32,
    ///[(scalapb.field).type = "coop.rchain.models.AlwaysEqual[scala.collection.immutable.BitSet]"];
    #[prost(message, optional, tag="6")]
    pub locally_free: ::core::option::Option<super::bitset::BitSet>,
    #[prost(bool, tag="7")]
    pub connective_used: bool,
}
/// Number of variables bound in the new statement.
/// For normalized form, p should not contain solely another new.
/// Also for normalized form, the first use should be level+0, next use level+1
/// up to level+count for the last used variable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct New {
    /// Includes any uris listed below. This makes it easier to substitute or walk a term.
    #[prost(sint32, tag="1")]
    pub bind_count: i32,
    #[prost(message, optional, tag="2")]
    pub p: ::core::option::Option<Par>,
    /// For normalization, uri-referenced variables come at the end, and in lexicographical order.
    #[prost(string, repeated, tag="3")]
    pub uri: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map="string, message", tag="4")]
    pub injections: ::std::collections::HashMap<::prost::alloc::string::String, Par>,
    ///[(scalapb.field).type = "coop.rchain.models.AlwaysEqual[scala.collection.immutable.BitSet]"];
    #[prost(message, optional, tag="5")]
    pub locally_free: ::core::option::Option<super::bitset::BitSet>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchCase {
    #[prost(message, optional, tag="1")]
    pub pattern: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub source: ::core::option::Option<Par>,
    #[prost(int32, tag="3")]
    pub free_count: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Match {
    #[prost(message, optional, tag="1")]
    pub target: ::core::option::Option<Par>,
    #[prost(message, repeated, tag="2")]
    pub cases: ::prost::alloc::vec::Vec<MatchCase>,
    ///[(scalapb.field).type = "coop.rchain.models.AlwaysEqual[scala.collection.immutable.BitSet]"];
    #[prost(message, optional, tag="4")]
    pub locally_free: ::core::option::Option<super::bitset::BitSet>,
    #[prost(bool, tag="5")]
    pub connective_used: bool,
}
/// Any process may be an operand to an expression.
/// Only processes equivalent to a ground process of compatible type will reduce.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Expr {
    #[prost(oneof="expr::ExprInstance", tags="1, 2, 3, 4, 25, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 27, 28, 29, 30, 31")]
    pub expr_instance: ::core::option::Option<expr::ExprInstance>,
}
/// Nested message and enum types in `Expr`.
pub mod expr {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ExprInstance {
        #[prost(bool, tag="1")]
        GBool(bool),
        #[prost(sint64, tag="2")]
        GInt(i64),
        #[prost(string, tag="3")]
        GString(::prost::alloc::string::String),
        #[prost(string, tag="4")]
        GUri(::prost::alloc::string::String),
        #[prost(bytes, tag="25")]
        GByteArray(::prost::alloc::vec::Vec<u8>),
        #[prost(message, tag="5")]
        ENotBody(super::ENot),
        #[prost(message, tag="6")]
        ENegBody(super::ENeg),
        #[prost(message, tag="7")]
        EMultBody(super::EMult),
        #[prost(message, tag="8")]
        EDivBody(super::EDiv),
        #[prost(message, tag="9")]
        EPlusBody(super::EPlus),
        #[prost(message, tag="10")]
        EMinusBody(super::EMinus),
        #[prost(message, tag="11")]
        ELtBody(super::ELt),
        #[prost(message, tag="12")]
        ELteBody(super::ELte),
        #[prost(message, tag="13")]
        EGtBody(super::EGt),
        #[prost(message, tag="14")]
        EGteBody(super::EGte),
        #[prost(message, tag="15")]
        EEqBody(super::EEq),
        #[prost(message, tag="16")]
        ENeqBody(super::ENeq),
        #[prost(message, tag="17")]
        EAndBody(super::EAnd),
        #[prost(message, tag="18")]
        EOrBody(super::EOr),
        #[prost(message, tag="19")]
        EVarBody(super::EVar),
        #[prost(message, tag="20")]
        EListBody(super::EList),
        #[prost(message, tag="21")]
        ETupleBody(super::ETuple),
        ///[(scalapb.field).type = "coop.rchain.models.ParSet"];
        #[prost(message, tag="22")]
        ESetBody(super::ESet),
        ///[(scalapb.field).type = "coop.rchain.models.ParMap"];
        #[prost(message, tag="23")]
        EMapBody(super::EMap),
        #[prost(message, tag="24")]
        EMethodBody(super::EMethod),
        #[prost(message, tag="27")]
        EMatchesBody(super::EMatches),
        /// string interpolation
        #[prost(message, tag="28")]
        EPercentPercentBody(super::EPercentPercent),
        /// concatenation
        #[prost(message, tag="29")]
        EPlusPlusBody(super::EPlusPlus),
        /// set difference
        #[prost(message, tag="30")]
        EMinusMinusBody(super::EMinusMinus),
        #[prost(message, tag="31")]
        EModBody(super::EMod),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EList {
    #[prost(message, repeated, tag="1")]
    pub ps: ::prost::alloc::vec::Vec<Par>,
    ///[(scalapb.field).type = "coop.rchain.models.AlwaysEqual[scala.collection.immutable.BitSet]"];
    #[prost(message, optional, tag="3")]
    pub locally_free: ::core::option::Option<super::bitset::BitSet>,
    #[prost(bool, tag="4")]
    pub connective_used: bool,
    #[prost(message, optional, tag="5")]
    pub remainder: ::core::option::Option<Var>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ETuple {
    #[prost(message, repeated, tag="1")]
    pub ps: ::prost::alloc::vec::Vec<Par>,
    ///[(scalapb.field).type = "coop.rchain.models.AlwaysEqual[scala.collection.immutable.BitSet]"];
    #[prost(message, optional, tag="3")]
    pub locally_free: ::core::option::Option<super::bitset::BitSet>,
    #[prost(bool, tag="4")]
    pub connective_used: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ESet {
    #[prost(message, repeated, tag="1")]
    pub ps: ::prost::alloc::vec::Vec<Par>,
    ///[(scalapb.field).type = "coop.rchain.models.AlwaysEqual[scala.collection.immutable.BitSet]"];
    #[prost(message, optional, tag="3")]
    pub locally_free: ::core::option::Option<super::bitset::BitSet>,
    #[prost(bool, tag="4")]
    pub connective_used: bool,
    #[prost(message, optional, tag="5")]
    pub remainder: ::core::option::Option<Var>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EMap {
    #[prost(message, repeated, tag="1")]
    pub kvs: ::prost::alloc::vec::Vec<KeyValuePair>,
    ///[(scalapb.field).type = "coop.rchain.models.AlwaysEqual[scala.collection.immutable.BitSet]"];
    #[prost(message, optional, tag="3")]
    pub locally_free: ::core::option::Option<super::bitset::BitSet>,
    #[prost(bool, tag="4")]
    pub connective_used: bool,
    #[prost(message, optional, tag="5")]
    pub remainder: ::core::option::Option<Var>,
}
///*
/// `target.method(arguments)`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EMethod {
    #[prost(string, tag="1")]
    pub method_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub target: ::core::option::Option<Par>,
    #[prost(message, repeated, tag="3")]
    pub arguments: ::prost::alloc::vec::Vec<Par>,
    ///[(scalapb.field).type = "coop.rchain.models.AlwaysEqual[scala.collection.immutable.BitSet]"];
    #[prost(message, optional, tag="5")]
    pub locally_free: ::core::option::Option<super::bitset::BitSet>,
    #[prost(bool, tag="6")]
    pub connective_used: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValuePair {
    #[prost(message, optional, tag="1")]
    pub key: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<Par>,
}
/// A variable used as a var should be bound in a process context, not a name
/// context. For example:
/// `for (@x <- c1; @y <- c2) { z!(x + y) }` is fine, but
/// `for (x <- c1; y <- c2) { z!(x + y) }` should raise an error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EVar {
    #[prost(message, optional, tag="1")]
    pub v: ::core::option::Option<Var>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ENot {
    #[prost(message, optional, tag="1")]
    pub p: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ENeg {
    #[prost(message, optional, tag="1")]
    pub p: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EMult {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EDiv {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EMod {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EPlus {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EMinus {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ELt {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ELte {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EGt {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EGte {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EEq {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ENeq {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EAnd {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EOr {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EMatches {
    #[prost(message, optional, tag="1")]
    pub target: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub pattern: ::core::option::Option<Par>,
}
///*
/// String interpolation
///
/// `"Hello, {name}" %% {"name": "Bob"}` denotes `"Hello, Bob"`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EPercentPercent {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
/// Concatenation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EPlusPlus {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
/// Set difference
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EMinusMinus {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<Par>,
    #[prost(message, optional, tag="2")]
    pub p2: ::core::option::Option<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connective {
    #[prost(oneof="connective::ConnectiveInstance", tags="1, 2, 3, 4, 5, 6, 7, 8, 9")]
    pub connective_instance: ::core::option::Option<connective::ConnectiveInstance>,
}
/// Nested message and enum types in `Connective`.
pub mod connective {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConnectiveInstance {
        #[prost(message, tag="1")]
        ConnAndBody(super::ConnectiveBody),
        #[prost(message, tag="2")]
        ConnOrBody(super::ConnectiveBody),
        #[prost(message, tag="3")]
        ConnNotBody(super::Par),
        #[prost(message, tag="4")]
        VarRefBody(super::VarRef),
        #[prost(bool, tag="5")]
        ConnBool(bool),
        #[prost(bool, tag="6")]
        ConnInt(bool),
        #[prost(bool, tag="7")]
        ConnString(bool),
        #[prost(bool, tag="8")]
        ConnUri(bool),
        #[prost(bool, tag="9")]
        ConnByteArray(bool),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VarRef {
    #[prost(sint32, tag="1")]
    pub index: i32,
    #[prost(sint32, tag="2")]
    pub depth: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectiveBody {
    #[prost(message, repeated, tag="1")]
    pub ps: ::prost::alloc::vec::Vec<Par>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployId {
    #[prost(bytes="vec", tag="1")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployerId {
    #[prost(bytes="vec", tag="1")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
}
/// Unforgeable names resulting from `new x { ... }`
/// These should only occur as the program is being evaluated. There is no way in
/// the grammar to construct them.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GUnforgeable {
    #[prost(oneof="g_unforgeable::UnfInstance", tags="1, 2, 3, 4")]
    pub unf_instance: ::core::option::Option<g_unforgeable::UnfInstance>,
}
/// Nested message and enum types in `GUnforgeable`.
pub mod g_unforgeable {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UnfInstance {
        #[prost(message, tag="1")]
        GPrivateBody(super::GPrivate),
        #[prost(message, tag="2")]
        GDeployIdBody(super::GDeployId),
        #[prost(message, tag="3")]
        GDeployerIdBody(super::GDeployerId),
        #[prost(message, tag="4")]
        GSysAuthTokenBody(super::GSysAuthToken),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GPrivate {
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GDeployId {
    #[prost(bytes="vec", tag="1")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GDeployerId {
    #[prost(bytes="vec", tag="1")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GSysAuthToken {
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SyntaxErrorKind {
    IntegerNumberError = 0,
    UnexpectedNameContext = 1,
    UnexpectedProcessContext = 2,
    UnexpectedReuseOfNameContextFree = 3,
    UnexpectedReuseOfProcessContextFree = 4,
    EmptyUri = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CompiliationErrorKind {
    UnrecognizedToken = 0,
    UnrecognizedData = 1,
    SystemError = 2,
    Utf8Error = 3,
    NullPointer = 4,
    UnsupportedVarSort = 5,
    InvalidFileHandle = 6,
    NotImplemented = 7,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionErrorKind {
    /// aborted because an error is detected
    Aborted = 0,
    InvalidExpression = 1,
    ArithmeticOverflow = 2,
}
