use std::fmt;
use std::path::Path;
use std::sync::Arc;

use libloading::{Library, Symbol};

/// Error type for library loading operations.
#[derive(Debug)]
pub enum LoadError {
    /// Failed to load the shared library itself.
    Library(libloading::Error),
    /// Failed to resolve a symbol from the loaded library.
    Symbol {
        name: String,
        source: libloading::Error,
    },
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Library(e) => write!(f, "failed to load library: {e}"),
            LoadError::Symbol { name, source } => {
                write!(f, "failed to load symbol `{name}`: {source}")
            }
        }
    }
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoadError::Library(e) => Some(e),
            LoadError::Symbol { source, .. } => Some(source),
        }
    }
}

/// Default NVENC library name for the current platform.
#[cfg(target_os = "windows")]
const NVENC_LIB_NAME: &str = "nvEncodeAPI64.dll";
#[cfg(target_os = "linux")]
const NVENC_LIB_NAME: &str = "libnvidia-encode.so.1";

/// Default CUDA driver library name for the current platform.
#[cfg(target_os = "windows")]
const CUDA_LIB_NAME: &str = "nvcuda.dll";
#[cfg(target_os = "linux")]
const CUDA_LIB_NAME: &str = "libcuda.so.1";

/// Default CUVID (NVDEC) library name for the current platform.
#[cfg(target_os = "windows")]
const CUVID_LIB_NAME: &str = "nvcuvid.dll";
#[cfg(target_os = "linux")]
const CUVID_LIB_NAME: &str = "libnvcuvid.so.1";

// ---------------------------------------------------------------------------
// NVENC Loader
// ---------------------------------------------------------------------------

/// Handle to the NVENC encoder library.
///
/// Loads `nvEncodeAPI64.dll` (Windows) or `libnvidia-encode.so.1` (Linux) and
/// provides access to the two NVENC entry points:
/// - `NvEncodeAPIGetMaxSupportedVersion`
/// - `NvEncodeAPICreateInstance`
///
/// The underlying library is kept alive via `Arc<Library>` so cloning this
/// handle is cheap and the library stays loaded as long as any handle exists.
pub struct NvencLib {
    lib: Arc<Library>,
}

impl NvencLib {
    /// Load the NVENC library from the default system path.
    pub fn load() -> Result<Self, LoadError> {
        let lib = unsafe { Library::new(NVENC_LIB_NAME) }.map_err(LoadError::Library)?;
        Ok(Self { lib: Arc::new(lib) })
    }

    /// Load the NVENC library from a specific path.
    pub fn load_from(path: impl AsRef<Path>) -> Result<Self, LoadError> {
        let lib = unsafe { Library::new(path.as_ref().as_os_str()) }.map_err(LoadError::Library)?;
        Ok(Self { lib: Arc::new(lib) })
    }

    /// Look up a symbol from the loaded NVENC library.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the type `T` matches the actual symbol type
    /// in the loaded library.
    pub unsafe fn get<T>(&self, name: &[u8]) -> Result<Symbol<'_, T>, LoadError> {
        unsafe { self.lib.get(name) }.map_err(|e| LoadError::Symbol {
            name: String::from_utf8_lossy(name).into_owned(),
            source: e,
        })
    }

    /// Returns a reference to the underlying `Library`.
    pub fn library(&self) -> &Library {
        &self.lib
    }

    /// Returns the `Arc<Library>` handle for shared ownership.
    pub fn library_arc(&self) -> Arc<Library> {
        Arc::clone(&self.lib)
    }
}

// ---------------------------------------------------------------------------
// CUDA Driver Loader
// ---------------------------------------------------------------------------

/// Handle to the CUDA driver library.
///
/// Loads `nvcuda.dll` (Windows) or `libcuda.so.1` (Linux) and provides access
/// to CUDA driver API symbols like `cuInit`, `cuCtxCreate_v2`, etc.
pub struct CudaLib {
    lib: Arc<Library>,
}

impl CudaLib {
    /// Load the CUDA driver library from the default system path.
    pub fn load() -> Result<Self, LoadError> {
        let lib = unsafe { Library::new(CUDA_LIB_NAME) }.map_err(LoadError::Library)?;
        Ok(Self { lib: Arc::new(lib) })
    }

    /// Load the CUDA driver library from a specific path.
    pub fn load_from(path: impl AsRef<Path>) -> Result<Self, LoadError> {
        let lib = unsafe { Library::new(path.as_ref().as_os_str()) }.map_err(LoadError::Library)?;
        Ok(Self { lib: Arc::new(lib) })
    }

    /// Look up a symbol from the loaded CUDA driver library.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the type `T` matches the actual symbol type
    /// in the loaded library.
    pub unsafe fn get<T>(&self, name: &[u8]) -> Result<Symbol<'_, T>, LoadError> {
        unsafe { self.lib.get(name) }.map_err(|e| LoadError::Symbol {
            name: String::from_utf8_lossy(name).into_owned(),
            source: e,
        })
    }

    /// Returns a reference to the underlying `Library`.
    pub fn library(&self) -> &Library {
        &self.lib
    }

    /// Returns the `Arc<Library>` handle for shared ownership.
    pub fn library_arc(&self) -> Arc<Library> {
        Arc::clone(&self.lib)
    }
}

// ---------------------------------------------------------------------------
// CUVID (NVDEC) Loader
// ---------------------------------------------------------------------------

/// Handle to the CUVID (NVDEC decoder) library.
///
/// Loads `nvcuvid.dll` (Windows) or `libnvcuvid.so.1` (Linux) and provides
/// access to CUVID decoder symbols like `cuvidCreateDecoder`,
/// `cuvidDecodePicture`, etc.
pub struct CuvidLib {
    lib: Arc<Library>,
}

impl CuvidLib {
    /// Load the CUVID library from the default system path.
    pub fn load() -> Result<Self, LoadError> {
        let lib = unsafe { Library::new(CUVID_LIB_NAME) }.map_err(LoadError::Library)?;
        Ok(Self { lib: Arc::new(lib) })
    }

    /// Load the CUVID library from a specific path.
    pub fn load_from(path: impl AsRef<Path>) -> Result<Self, LoadError> {
        let lib = unsafe { Library::new(path.as_ref().as_os_str()) }.map_err(LoadError::Library)?;
        Ok(Self { lib: Arc::new(lib) })
    }

    /// Look up a symbol from the loaded CUVID library.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the type `T` matches the actual symbol type
    /// in the loaded library.
    pub unsafe fn get<T>(&self, name: &[u8]) -> Result<Symbol<'_, T>, LoadError> {
        unsafe { self.lib.get(name) }.map_err(|e| LoadError::Symbol {
            name: String::from_utf8_lossy(name).into_owned(),
            source: e,
        })
    }

    /// Returns a reference to the underlying `Library`.
    pub fn library(&self) -> &Library {
        &self.lib
    }

    /// Returns the `Arc<Library>` handle for shared ownership.
    pub fn library_arc(&self) -> Arc<Library> {
        Arc::clone(&self.lib)
    }
}
