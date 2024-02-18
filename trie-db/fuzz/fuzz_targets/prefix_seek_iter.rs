#![no_main]

use libfuzzer_sys::fuzz_target;
use trie_db_fuzz::{fuzz_prefix_seek_iter, PrefixSeekTestInput};

fuzz_target!(|data: PrefixSeekTestInput| {
	fuzz_prefix_seek_iter::<reference_trie_fun::SubstrateV1<reference_trie_fun::RefHasher>>(data);
});
