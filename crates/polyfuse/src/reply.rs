use self::non_exhaustive::NonExhaustive;
use crate::types::{FileAttr, FsStatistics};
use std::{ffi::OsStr, io, time::Duration};

/// The error values caused by the filesystem.
pub trait Error {
    /// Construct the error value from an I/O error.
    fn from_io_error(io_error: io::Error) -> Self;

    /// Construct the error value from an OS error code.
    #[inline]
    fn from_code(code: i32) -> Self
    where
        Self: Sized,
    {
        Self::from_io_error(io::Error::from_raw_os_error(code))
    }
}

/// Equivalent to `<E as Error>::from_code(code)`.
#[inline]
pub fn error_code<E>(code: i32) -> E
where
    E: Error,
{
    E::from_code(code)
}

pub trait ReplyEntry {
    type Ok;
    type Error: Error;

    fn entry<T>(self, attr: T, opts: &EntryOptions) -> Result<Self::Ok, Self::Error>
    where
        T: FileAttr;
}

/// The option values for `ReplyEntry`.
#[derive(Copy, Clone, Debug)]
pub struct EntryOptions {
    /// Return the inode number of this entry.
    ///
    /// If this value is zero, it means that the entry is *negative*.
    /// Returning a negative entry is also possible with the `ENOENT` error,
    /// but the *zeroed* entries also have the ability to specify the lifetime
    /// of the entry cache by using the `ttl_entry` parameter.
    ///
    /// The default value is `0`.
    pub ino: u64,

    /// Return the validity timeout for inode attributes.
    ///
    /// The operations should set this value to very large
    /// when the changes of inode attributes are caused
    /// only by FUSE requests.
    pub ttl_attr: Option<Duration>,

    /// Return the validity timeout for the name.
    ///
    /// The operations should set this value to very large
    /// when the changes/deletions of directory entries are
    /// caused only by FUSE requests.
    pub ttl_entry: Option<Duration>,

    /// Return the generation of this entry.
    ///
    /// This parameter is used to distinguish the inode from the past one
    /// when the filesystem reuse inode numbers.  That is, the operations
    /// must ensure that the pair of entry's inode number and generation
    /// are unique for the lifetime of the filesystem.
    pub generation: u64,

    #[doc(hidden)] // non_exhaustive
    pub __non_exhaustive: NonExhaustive,
}

impl Default for EntryOptions {
    fn default() -> Self {
        Self {
            ino: 0,
            ttl_attr: None,
            ttl_entry: None,
            generation: 0,

            __non_exhaustive: NonExhaustive::INIT,
        }
    }
}

pub trait ReplyAttr {
    type Ok;
    type Error: Error;

    fn attr<T>(self, attr: T, ttl: Option<Duration>) -> Result<Self::Ok, Self::Error>
    where
        T: FileAttr;
}

pub trait ReplyOk {
    type Ok;
    type Error: Error;

    fn ok(self) -> Result<Self::Ok, Self::Error>;
}

pub trait ReplyData {
    type Ok;
    type Error: Error;

    fn data<T>(self, data: T) -> Result<Self::Ok, Self::Error>
    where
        T: AsRef<[u8]>;
}

pub trait ReplyOpen {
    type Ok;
    type Error: Error;

    fn open(self, fh: u64, opts: &OpenOptions) -> Result<Self::Ok, Self::Error>;
}

/// The option values for `ReplyOpen`.
#[derive(Copy, Clone, Debug)]
pub struct OpenOptions {
    /// Indicates that the direct I/O is used on this file.
    pub direct_io: bool,

    /// Indicates that the currently cached file data in the kernel
    /// need not be invalidated.
    pub keep_cache: bool,

    /// Indicates that the opened file is not seekable.
    pub nonseekable: bool,

    /// Enable caching of entries returned by `readdir`.
    ///
    /// This flag is meaningful only for `opendir` operations.
    pub cache_dir: bool,

    #[doc(hidden)] // non_exhaustive
    pub __non_exhaustive: NonExhaustive,
}

impl Default for OpenOptions {
    fn default() -> Self {
        Self {
            direct_io: false,
            keep_cache: false,
            nonseekable: false,
            cache_dir: false,

            __non_exhaustive: NonExhaustive::INIT,
        }
    }
}

pub trait ReplyWrite {
    type Ok;
    type Error: Error;

    fn size(self, size: u32) -> Result<Self::Ok, Self::Error>;
}

pub trait ReplyStatfs {
    type Ok;
    type Error: Error;

    fn stat<S>(self, stat: S) -> Result<Self::Ok, Self::Error>
    where
        S: FsStatistics;
}

pub trait ReplyXattr {
    type Ok;
    type Error: Error;

    fn size(self, size: u32) -> Result<Self::Ok, Self::Error>;

    fn data<T>(self, data: T) -> Result<Self::Ok, Self::Error>
    where
        T: AsRef<[u8]>;
}

pub trait ReplyLk {
    type Ok;
    type Error: Error;

    fn lk(self, typ: u32, start: u64, end: u64, pid: u32) -> Result<Self::Ok, Self::Error>;
}

pub trait ReplyCreate {
    type Ok;
    type Error: Error;

    fn create<T>(
        self,
        fh: u64,
        attr: T,
        entry_opts: &EntryOptions,
        open_opts: &OpenOptions,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: FileAttr;
}

pub trait ReplyBmap {
    type Ok;
    type Error: Error;

    fn block(self, block: u64) -> Result<Self::Ok, Self::Error>;
}

pub trait ReplyPoll {
    type Ok;
    type Error: Error;

    fn revents(self, revents: u32) -> Result<Self::Ok, Self::Error>;
}

pub trait ReplyDirs {
    type Ok;
    type Error: Error;

    fn add<D>(&mut self, dirent: D, offset: u64) -> bool
    where
        D: DirEntry;

    fn send(self) -> Result<Self::Ok, Self::Error>;
}

pub trait ReplyDirsPlus {
    type Ok;
    type Error: Error;

    fn add<D, A>(&mut self, dirent: D, offset: u64, attr: A, opts: &EntryOptions) -> bool
    where
        D: DirEntry,
        A: FileAttr;

    fn send(self) -> Result<Self::Ok, Self::Error>;
}

/// A directory entry replied to the kernel.
pub trait DirEntry {
    /// Return the inode number of this entry.
    fn ino(&self) -> u64;

    /// Return the type of this entry.
    fn typ(&self) -> u32;

    /// Returns the name of this entry.
    fn name(&self) -> &OsStr;
}

mod non_exhaustive {
    #[derive(Copy, Clone, Debug)]
    pub struct NonExhaustive(());

    impl NonExhaustive {
        pub(crate) const INIT: Self = Self(());
    }
}
