/// `funclock_fs` - A simple lock file manager for session-based locking. made specifically for
/// projects like [MCServer](https://github.com/lordofwizard/mcserver)
///
/// This crate provides a `LockFile` struct that allows you to manage lock files
/// to prevent concurrent access to resources. The lock file is created in the
/// `/tmp` directory with a specified session name.
use std::fs;
use std::path::PathBuf;

/// A struct to manage a lock file for a given session.
pub struct LockFile {
    lockfile_path: PathBuf,
}

impl LockFile {
    /// Creates a new `LockFile` instance with the specified session name.
    ///
    /// # Arguments
    ///
    /// * `session_name` - A string slice that holds the name of the session.
    ///
    /// # Example
    ///
    /// ```
    /// let lockfile = funclock_fs::LockFile::new("my_session");
    /// ```
    pub fn new(session_name: &str) -> Self {
        let mut lockfile_path = PathBuf::from("/tmp");
        lockfile_path.push(format!("{}.lock", session_name));

        LockFile { lockfile_path }
    }

    /// Checks if the lock file currently exists.
    ///
    /// # Returns
    ///
    /// * `true` if the lock file exists, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let lockfile = funclock_fs::LockFile::new("my_session");
    /// if lockfile.is_locked() {
    ///     println!("Lock file exists.");
    /// } else {
    ///     println!("Lock file does not exist.");
    /// }
    /// ```
    pub fn is_locked(&self) -> bool {
        self.lockfile_path.exists()
    }

    /// Creates a lock file if it does not already exist.
    ///
    /// If the lock file already exists, a message will be printed indicating
    /// that the file is already present.
    ///
    /// # Example
    ///
    /// ```
    /// let lockfile = funclock_fs::LockFile::new("my_session");
    /// lockfile.lock();
    /// ```
    pub fn lock(&self) {
        if self.is_locked() {
            println!("Lock file already exists.");
        } else {
            match fs::write(&self.lockfile_path, "Locked") {
                Ok(_) => println!("Lock file created successfully."),
                Err(e) => println!("Error creating lock file: {}", e),
            }
        }
    }

    /// Removes the lock file if it exists.
    ///
    /// If no lock file exists, a message will be printed indicating that
    /// there is no file to remove.
    ///
    /// # Example
    ///
    /// ```
    /// let lockfile = funclock_fs::LockFile::new("my_session");
    /// lockfile.release();
    /// ```
    pub fn release(&self) {
        if self.is_locked() {
            match fs::remove_file(&self.lockfile_path) {
                Ok(_) => println!("Lock file removed successfully."),
                Err(e) => println!("Error removing lock file: {}", e),
            }
        } else {
            println!("No lock file exists to release.");
        }
    }
}

