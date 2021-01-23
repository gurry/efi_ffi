use ffi::base::{
    EFI_GUID, 
    EFI_STATUS, 
    EFI_EVENT,
    EFI_TIME,
    UINTN,
    UINT32,
    UINT64,
    CHAR16,
    BOOLEAN,
    VOID,
};

use super::device_path::EFI_DEVICE_PATH_PROTOCOL;

pub const EFI_LOAD_FILE_PROTOCOL_GUID: EFI_GUID = EFI_GUID(0x56EC3091, 0x954C, 0x11d2, [0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B]);

#[derive(Clone)]
#[repr(C)]
pub struct EFI_LOAD_FILE_PROTOCOL {
    pub LoadFile: EFI_LOAD_FILE
}

pub type EFI_LOAD_FILE = extern "win64" fn(
    This: *const EFI_LOAD_FILE_PROTOCOL, 
    FilePath: *const EFI_DEVICE_PATH_PROTOCOL,
    BootPolicy: BOOLEAN,
    BufferSize: *mut UINTN, 
    BufferPtr: *mut VOID
) -> EFI_STATUS;

pub const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID: EFI_GUID  = EFI_GUID(0x0964E5B22, 0x6459, 0x11D2, [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B]);

pub const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_REVISION: UINT64 = 0x00010000;

#[derive(Clone)]
#[repr(C)]
pub struct EFI_SIMPLE_FILE_SYSTEM_PROTOCOL {
    pub Revision: UINT64,
    pub OpenVolume: EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_OPEN_VOLUME,
}

pub type EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_OPEN_VOLUME = extern "win64" fn(
    This: *const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL,
    Root: *mut *const EFI_FILE_PROTOCOL
) -> EFI_STATUS;

pub const EFI_FILE_PROTOCOL_REVISION: UINT64 = 0x00010000;
pub const EFI_FILE_PROTOCOL_REVISION2: UINT64 = 0x00020000;
pub const EFI_FILE_PROTOCOL_LATEST_REVISION: UINT64 = EFI_FILE_PROTOCOL_REVISION2;

#[derive(Clone)]
#[repr(C)]
pub struct EFI_FILE_PROTOCOL {
    pub Revision: UINT64,
    pub Open: EFI_FILE_OPEN,
    pub Close: EFI_FILE_CLOSE,
    pub Delete: EFI_FILE_DELETE,
    pub Read: EFI_FILE_READ,
    pub Write: EFI_FILE_WRITE,
    pub GetPosition: EFI_FILE_GET_POSITION,
    pub SetPosition: EFI_FILE_SET_POSITION,
    pub GetInfo: EFI_FILE_GET_INFO,
    pub SetInfo: EFI_FILE_SET_INFO,
    pub Flush: EFI_FILE_FLUSH,
    pub OpenEx: EFI_FILE_OPEN_EX,
    pub ReadEx: EFI_FILE_READ_EX,
    pub WriteEx: EFI_FILE_WRITE_EX,
    pub FlushEx: EFI_FILE_FLUSH_EX,
}

pub type EFI_FILE_OPEN = extern "win64" fn(
    This: *mut EFI_FILE_PROTOCOL,
    NewHandle: *mut *const EFI_FILE_PROTOCOL,
    FileName: *const CHAR16,
    OpenMode: UINT64,
    Attribute: UINT64
) -> EFI_STATUS;

pub type EFI_FILE_CLOSE = extern "win64" fn(
    This: *mut EFI_FILE_PROTOCOL
) -> EFI_STATUS;

pub type EFI_FILE_DELETE = extern "win64" fn(
    This: *mut EFI_FILE_PROTOCOL
) -> EFI_STATUS;

pub type EFI_FILE_READ = extern "win64" fn(
    This: *mut EFI_FILE_PROTOCOL,
    BufferSize: *mut UINTN,
    Buffer: *mut VOID
) -> EFI_STATUS;

pub type EFI_FILE_WRITE = extern "win64" fn(
    This: *mut EFI_FILE_PROTOCOL,
    BufferSize: *mut UINTN,
    Buffer: *const VOID
) -> EFI_STATUS;

pub type EFI_FILE_GET_POSITION = extern "win64" fn(
    This: *const EFI_FILE_PROTOCOL,
    Position: *mut UINT64
) -> EFI_STATUS;

pub type EFI_FILE_SET_POSITION = extern "win64" fn(
    This: *mut EFI_FILE_PROTOCOL,
    Position: UINT64
) -> EFI_STATUS;

pub type EFI_FILE_GET_INFO = extern "win64" fn(
    This: *const EFI_FILE_PROTOCOL,
    InformationType: *const EFI_GUID,
    BufferSize: *mut UINTN,
    Buffer: *mut VOID
) -> EFI_STATUS;

pub type EFI_FILE_SET_INFO = extern "win64" fn(
    This: *mut EFI_FILE_PROTOCOL,
    InformationType: *const EFI_GUID,
    BufferSize: UINTN,
    Buffer: *const VOID
) -> EFI_STATUS;

pub type EFI_FILE_FLUSH = extern "win64" fn(
    This: *mut EFI_FILE_PROTOCOL
) -> EFI_STATUS;

pub type EFI_FILE_OPEN_EX =  extern "win64" fn(
    This: *mut EFI_FILE_PROTOCOL,
    NewHandle: *mut *const EFI_FILE_PROTOCOL,
    FileName: *const CHAR16,
    OpenMode: UINT64,
    Attributes: UINT64,
    Token: *mut EFI_FILE_IO_TOKEN
) -> EFI_STATUS;

pub type EFI_FILE_READ_EX = extern "win64" fn(
    This: *mut EFI_FILE_PROTOCOL,
    Token: *mut EFI_FILE_IO_TOKEN
) -> EFI_STATUS;

pub type EFI_FILE_WRITE_EX = extern "win64" fn(
    This: *mut EFI_FILE_PROTOCOL,
    Token: *mut EFI_FILE_IO_TOKEN
) -> EFI_STATUS;

pub type EFI_FILE_FLUSH_EX = extern "win64" fn(
    This: *mut EFI_FILE_PROTOCOL,
    Token: *mut EFI_FILE_IO_TOKEN
) -> EFI_STATUS;

#[derive(Debug)]
#[repr(C)]
pub struct EFI_FILE_IO_TOKEN {
  //
  // If Event is NULL, then blocking I/O is performed.
  // If Event is not NULL and non-blocking I/O is supported, then non-blocking I/O is performed,
  // and Event will be signaled when the read request is completed.
  // The caller must be prepared to handle the case where the callback associated with Event
  // occurs before the original asynchronous I/O request call returns.
  //
  pub Event: EFI_EVENT,

  //
  // Defines whether or not the signaled event encountered an error.
  //
  pub Status: EFI_STATUS,

  //
  // For OpenEx():  Not Used, ignored.
  // For ReadEx():  On input, the size of the Buffer. On output, the amount of data returned in Buffer.
  //                In both cases, the size is measured in bytes.
  // For WriteEx(): On input, the size of the Buffer. On output, the amount of data actually written.
  //                In both cases, the size is measured in bytes.
  // For FlushEx(): Not used, ignored.
  //
  pub BufferSize: UINTN,

  //
  // For OpenEx():  Not Used, ignored.
  // For ReadEx():  The buffer into which the data is read.
  // For WriteEx(): The buffer of data to write.
  // For FlushEx(): Not Used, ignored.
  //
  pub Buffer: *mut VOID,
}

pub const EFI_FILE_INFO_ID: EFI_GUID = EFI_GUID(0x09576E92, 0x6D3F, 0x11D2, [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B]);
                                                // 0x09576e92, 0x6d3f, 0x11d2,  0x8e39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b
#[derive(Debug)]
#[repr(C)]
pub struct EFI_FILE_INFO {
    pub Size: UINT64,
    pub FileSize: UINT64,
    pub PhysicalSize: UINT64,
    pub CreateTime: EFI_TIME,
    pub LastAccessTime: EFI_TIME,
    pub ModificationTime: EFI_TIME,
    pub Attribute: UINT64,
    pub FileName: [CHAR16; 1], // Dynamically sized, null-terminated embedded string
}

pub const EFI_FILE_MODE_READ: UINT64 = 0x0000000000000001;
pub const EFI_FILE_MODE_WRITE: UINT64 = 0x0000000000000002;
pub const EFI_FILE_MODE_CREATE: UINT64 = 0x8000000000000000;

pub const EFI_FILE_READ_ONLY: UINT64 = 0x0000000000000001;
pub const EFI_FILE_HIDDEN: UINT64 = 0x0000000000000002;
pub const EFI_FILE_SYSTEM: UINT64 = 0x0000000000000004;
pub const EFI_FILE_RESERVED: UINT64 = 0x0000000000000008;
pub const EFI_FILE_DIRECTORY: UINT64 = 0x0000000000000010;
pub const EFI_FILE_ARCHIVE: UINT64 = 0x0000000000000020;
pub const EFI_FILE_VALID_ATTR: UINT64 = 0x0000000000000037;

pub const EFI_FILE_SYSTEM_INFO_ID: EFI_GUID = EFI_GUID(0x09576E93, 0x6D3F, 0x11D2, [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B]);

#[derive(Debug)]
#[repr(C)]
pub struct EFI_FILE_SYSTEM_INFO {
    pub Size: UINT64,
    pub ReadOnly: BOOLEAN,
    pub VolumeSize: UINT64,
    pub FreeSpace: UINT64,
    pub BlockSize: UINT32,
    pub VolumeLabel: [CHAR16; 1], // Dynamically sized, null-terminated embedded string
}