# funclock_fs

`funclock_fs` is a Rust crate designed to manage lock files for session-based locking. It provides a simple way to prevent concurrent access to resources by creating and managing lock files in the `/tmp` directory.

## Features

- Create lock files with a specified session name.
- Check if a lock file exists.
- Create a lock file if it doesn't exist.
- Remove a lock file when no longer needed.

## Installation

Add `funclock_fs` to your `Cargo.toml` file:

```toml
[dependencies]
funclock_fs = "0.1"  # Replace with the latest version
```

## Usage

Here’s a quick example of how to use `funclock_fs`:

```rust
use funclock_fs::LockFile;

fn main() {
    // Create a new LockFile instance with the session name "my_session"
    let lockfile = LockFile::new("my_session");

    // Check if the lock file exists
    if lockfile.is_locked() {
        println!("Lock file exists.");
    } else {
        println!("Lock file does not exist.");
    }

    // Create the lock file
    lockfile.lock();

    // Release the lock file
    lockfile.release();
}
```

## API Documentation

The API documentation for `funclock_fs` is available on [docs.rs](https://docs.rs/funclock_fs). You can also view the documentation by running:

```sh
cargo doc --open
```

## Contributing

Contributions are welcome! Please fork the repository on [GitHub](https://github.com/lordofwizard/funclock_fs) and submit a pull request with your changes.

## License

`funclock_fs` is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgements

- The crate is inspired by common practices in file locking mechanisms.
- Special thanks to the Rust community for their valuable contributions and support.

