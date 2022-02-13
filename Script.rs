pub struct Script(_);

impl Script
Trait Implementations
[src]
impl AsRef<[u8]> for Script
[src]
fn as_ref(&self) -> &[u8]ⓘ
Performs the conversion.

[src]
impl Clone for Script
[src]
fn clone(&self) -> Script
Returns a copy of the value. Read more

1.0.0[src]
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more

[src]
impl Debug for Script
[src]
fn fmt(&self, f: &mut Formatter<'_>) -> Result
Formats the value using the given formatter. Read more

[src]
impl Decodable for Script
[src]
fn consensus_decode<D: Read>(d: D) -> Result<Self, Error>
Decode an object with a well-defined format

[src]
impl Default for Script
[src]
fn default() -> Script
Returns the “default value” for a type. Read more

[src]
impl Deserialize for Script
[src]
fn deserialize(bytes: &[u8]) -> Result<Self, Error>
Deserialize a value from raw data.

[src]
impl Display for Script
[src]
fn fmt(&self, f: &mut Formatter<'_>) -> Result
Formats the value using the given formatter. Read more

[src]
impl Encodable for Script
[src]
fn consensus_encode<S: Write>(&self, s: S) -> Result<usize, Error>
Encode an object with a well-defined format. Returns the number of bytes written on success. Read more

[src]
impl From<Vec<u8, Global>> for Script
Creates a new script from an existing vector

[src]
fn from(v: Vec<u8>) -> Script
Performs the conversion.

[src]
impl FromHex for Script
[src]
fn from_byte_iter<I>(iter: I) -> Result<Self, Error>
where
    I: Iterator<Item = Result<u8, Error>> + ExactSizeIterator + DoubleEndedIterator, 
Produce an object from a byte iterator

[src]
fn from_hex(s: &str) -> Result<Self, Error>
Produce an object from a hex string

[src]
impl FromStr for Script
type Err = Error
The associated error which can be returned from parsing.

[src]
fn from_str(s: &str) -> Result<Self, Error>
Parses a string s to return a value of this type. Read more

[src]
impl Hash for Script
[src]
fn hash<__H: Hasher>(&self, state: &mut __H)
Feeds this value into the given Hasher. Read more

1.3.0[src]
fn hash_slice<H>(data: &[Self], state: &mut H)
where
    H: Hasher, 
Feeds a slice of this type into the given Hasher. Read more

[src]
impl Index<Range<usize>> for Script
type Output = [u8]
The returned type after indexing.

[src]
fn index(&self, index: Range<usize>) -> &[u8]ⓘ
Performs the indexing (container[index]) operation. Read more

[src]
impl Index<RangeFrom<usize>> for Script
type Output = [u8]
The returned type after indexing.

[src]
fn index(&self, index: RangeFrom<usize>) -> &[u8]ⓘ
Performs the indexing (container[index]) operation. Read more

[src]
impl Index<RangeFull> for Script
type Output = [u8]
The returned type after indexing.

[src]
fn index(&self, _: RangeFull) -> &[u8]ⓘ
Performs the indexing (container[index]) operation. Read more

[src]
impl Index<RangeTo<usize>> for Script
type Output = [u8]
The returned type after indexing.

[src]
fn index(&self, index: RangeTo<usize>) -> &[u8]ⓘ
Performs the indexing (container[index]) operation. Read more

[src]
impl Index<usize> for Script
type Output = u8
The returned type after indexing.

[src]
fn index(&self, index: usize) -> &u8
Performs the indexing (container[index]) operation. Read more

[src]
impl LowerHex for Script
[src]
fn fmt(&self, f: &mut Formatter<'_>) -> Result
Formats the value using the given formatter.

[src]
impl Ord for Script
[src]
fn cmp(&self, other: &Script) -> Ordering
This method returns an Ordering between self and other. Read more

1.21.0[src]
fn max(self, other: Self) -> Self
Compares and returns the maximum of two values. Read more

1.21.0[src]
fn min(self, other: Self) -> Self
Compares and returns the minimum of two values. Read more

1.50.0[src]
fn clamp(self, min: Self, max: Self) -> Self
Restrict a value to a certain interval. Read more

[src]
impl PartialEq<Script> for Script
[src]
fn eq(&self, other: &Script) -> bool
This method tests for self and other values to be equal, and is used by ==. Read more

[src]
fn ne(&self, other: &Script) -> bool
This method tests for !=.

[src]
impl PartialOrd<Script> for Script
[src]
fn partial_cmp(&self, other: &Script) -> Option<Ordering>
This method returns an ordering between self and other values if one exists. Read more

1.0.0[src]
fn lt(&self, other: &Rhs) -> bool
This method tests less than (for self and other) and is used by the < operator. Read more

1.0.0[src]
fn le(&self, other: &Rhs) -> bool
This method tests less than or equal to (for self and other) and is used by the <= operator. Read more

1.0.0[src]
fn gt(&self, other: &Rhs) -> bool
This method tests greater than (for self and other) and is used by the > operator. Read more

1.0.0[src]
fn ge(&self, other: &Rhs) -> bool
This method tests greater than or equal to (for self and other) and is used by the >= operator. Read more

[src]
impl Serialize for Script
[src]
fn serialize(&self) -> Vec<u8>ⓘ
Serialize a value as raw data.

[src]
impl UpperHex for Script
[src]
fn fmt(&self, f: &mut Formatter<'_>) -> Result
Formats the value using the given formatter.

[src]
impl Eq for Script
[src]
impl StructuralEq for Script
[src]
impl StructuralPartialEq for Script
