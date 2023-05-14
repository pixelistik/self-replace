use std::env;
use std::fs;
use std::io;
use std::path::Path;

/// On Unix a running executable can be safely deleted.
pub fn self_delete() -> Result<(), io::Error> {
    let exe = env::current_exe()?.canonicalize()?;
    fs::remove_file(&exe)?;
    Ok(())
}

pub fn self_replace(new_executable: &Path) -> Result<(), io::Error> {
    let exe = env::current_exe()?;

    let tmp = tempfile::Builder::new()
        .prefix("._tempexeswap")
        .tempfile_in(&exe.parent().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "executable has no known parent folder",
            )
        })?)?;
    fs::copy(&new_executable, tmp.path())?;

    // if we made it this far, try to persist the temporary file and move it over.
    let (_, path) = tmp.keep()?;
    match fs::rename(&path, &new_executable) {
        Ok(()) => {}
        Err(err) => {
            fs::remove_file(&path).ok();
            return Err(err);
        }
    }

    // TODO: copy old mode and permissions to the new file.

    Ok(())
}