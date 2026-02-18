// Wrapper header for bindgen - includes all NVIDIA Video Codec SDK headers
// from FFmpeg/nv-codec-headers.
//
// We deliberately exclude dynlink_loader.h because we provide our own Rust-native
// dynamic loader (see src/loader.rs).

#include "ffnvcodec/dynlink_cuda.h"
#include "ffnvcodec/nvEncodeAPI.h"
#include "ffnvcodec/dynlink_cuviddec.h"
#include "ffnvcodec/dynlink_nvcuvid.h"
