fn main() {
    windows::build!(
        windows::win32::shell::*, windows::storage::UserDataPaths
    );
}
