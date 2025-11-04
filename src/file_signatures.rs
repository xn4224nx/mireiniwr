/*
 * Functions associated with file signitures or magic numbers.
 */

enum FileSigniture {
    Unknown,
    MultiBitBitcoinWallet,
    ArmoredPGPPublicKey,
    SQLiteDatabase,
    TelegramDesktopFile,
    TelegramDesktopEncryptedFile,
    JKSJavaKeyStore,
    PuTTYPrivateKeyV2,
    PuTTYPrivateKeyV3,
    OpenSSHPrivateKey,
    WindowsRegistry,
}

impl FileSigniture {
    fn from_bytes(inital_file_bytes: &Vec<u8>) -> Self {
        FileSigniture::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_multibit_bitcoin_wallet() {}

    #[test]
    fn detect_armored_pgp_public_key() {}

    #[test]
    fn detect_sqlite_database() {}

    #[test]
    fn detect_telegram_desktop_file() {}

    #[test]
    fn detect_telegram_desktop_encrypted_file() {}

    #[test]
    fn detect_jks_java_key_store() {}

    #[test]
    fn detect_putty_private_key_v2() {}

    #[test]
    fn detect_putty_private_key_v3() {}

    #[test]
    fn detect_open_ssh_private_key() {}

    #[test]
    fn detect_windows_registry() {}
}
