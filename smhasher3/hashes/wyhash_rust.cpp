/*
 * ###YOURHASHNAME
 * Copyright (C) 2022 ###YOURNAME
 *
 * ###YOURLICENSETEXT
 */
#include "Hashlib.h"
#include "Platform.h"

// XXX Your hash filename MUST end in .cpp, and it MUST start with a
// lowercase letter!
//
// XXX Don't forget to add your new filename to the list in
// hashes/Hashsrc.cmake, keeping the list sorted by size!

//------------------------------------------------------------
// ###YOURHASHCODE

//------------------------------------------------------------
template <bool bswap>
static void Wyhash_rust(const void *in, const size_t len, const seed_t seed,
                        void *out) {
  // extern "C" {
  // uint64_t hash = wyhash_3_64(in, len, seed);
  uint64_t hash = 0;
  // }
  PUT_U64<bswap>(hash, (uint8_t *)out, 0);
}

//------------------------------------------------------------
REGISTER_FAMILY(wyhash_rust, 
   $.src_url = "https://github.com/wangyi-fudan/wyhash",
   $.src_status = HashFamilyInfo::SRC_ACTIVE
);

REGISTER_HASH(wyhash_rust,
   $.desc = "wyhash rust",
   $.hash_flags =
     0,
   $.impl_flags =
         FLAG_IMPL_MULTIPLY_64_128  |
         FLAG_IMPL_ROTATE           |
         FLAG_IMPL_LICENSE_PUBLIC_DOMAIN,
   $.bits = 64,
   $.verification_LE = 0x9DAE7DD3,
   $.verification_BE = 0x2E958F8A,
   $.hashfn_native = Wyhash_rust<false>,
   $.hashfn_bswap = Wyhash_rust<true>
);
