extern crate alloc;

use icu::{
    collator::{Collator, CollatorOptions},
    locid::Locale,
};

struct UnstableProvider;
include!("./icu-data/mod.rs");
impl_data_provider!(UnstableProvider);

fn main() {
    let locale = Locale::try_from_bytes("en-US".as_bytes()).unwrap();
    let provider = UnstableProvider;
    let opts = CollatorOptions::new();
    Collator::try_new_unstable(&provider, &locale.into(), opts).unwrap();
}
