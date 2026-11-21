//! Run at login. TODO(phase 10): with MSIX use a StartupTask in the manifest;
//! for unpackaged builds write HKCU\...\Run.
pub fn set_run_at_startup(_enabled: bool) -> std::io::Result<()> {
    Ok(())
}
