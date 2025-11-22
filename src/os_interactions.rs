/*
 * Entities For Dealing With The OS
 */

use std::error::Error;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Using the path to a file, extract the at least the first 64 bytes of its data
fn read_file_header(file: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
    let num_bytes = 64;

    /* Open the file and move the pointer to the position to read from. */
    let mut f_pntr = std::fs::File::open(file)?;

    /* Determine if the file is big enough to fill the whole buffer. */
    let file_size: usize = f_pntr.metadata().unwrap().len() as usize;
    let buf_size: usize = std::cmp::min(file_size, num_bytes);
    let mut buffer = vec![0; buf_size];

    /* Read the data from the file and return it. */
    f_pntr.read_exact(&mut buffer)?;
    return Ok(buffer);
}

/// Return all the relative paths of files with specific extentions or that are
/// text files recursively in a specific directory.
fn file_search(
    directory: &Path,
    extentions: &Vec<String>,
    txt_files: bool,
) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut found_paths = Vec::new();

    /* Ensure the supplied path is valid and accessible. */
    match std::fs::metadata(directory) {
        Ok(metadata) => {
            if !metadata.is_dir() {
                return Err(std::io::ErrorKind::NotADirectory.into());
            }
        }
        Err(error) => {
            return Err(error);
        }
    }

    /* Iterate over the directory contents saving paths that match the extentions. */
    'path_walk: for dir_enity in WalkDir::new(directory).into_iter().filter_map(|x| x.ok()) {
        let entry = dir_enity.path();

        if entry.is_file() {
            for exten in extentions.iter() {
                if (entry.extension().is_some() && *entry.extension().unwrap() == **exten)
                    || (entry.extension().is_none() && *exten == String::from(""))
                {
                    found_paths.push(entry.to_path_buf());
                    continue 'path_walk;
                }
            }

            /* The file is not text if it has non-printible chars. */
            if txt_files {
                let Ok(file_head) = read_file_header(entry) else {
                    continue 'path_walk;
                };

                /* Ignore empty files. */
                if file_head.len() == 0 {
                    continue 'path_walk;
                };

                /* Check for non-printable chars. */
                for char_val in file_head.into_iter() {
                    if char_val < 32 || char_val == 127 {
                        continue 'path_walk;
                    }
                }
                found_paths.push(entry.to_path_buf());
            }
        }
    }
    return Ok(found_paths);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn read_empty_file() {
        assert_eq!(
            read_file_header(&Path::new(
                "./tests/testing_files/read_file_header/empty_file.txt"
            ))
            .unwrap(),
            Vec::new()
        )
    }

    #[test]
    fn read_file_smaller_than_64_bytes() {
        assert_eq!(
            read_file_header(&Path::new(
                "./tests/testing_files/read_file_header/smaller_64_bytes.csv"
            ))
            .unwrap(),
            vec![
                0x4E, 0x61, 0x6D, 0x65, 0x20, 0x2C, 0x41, 0x67, 0x65, 0x2C, 0x48, 0x65, 0x69, 0x67,
                0x68, 0x74, 0x0A, 0x4D, 0x61, 0x72, 0x6B, 0x2C, 0x32, 0x32, 0x2C, 0x31, 0x2E, 0x36,
                0x35, 0x0A
            ]
        )
    }

    #[test]
    fn read_non_existant_file() {
        assert!(
            read_file_header(&Path::new(
                "./tests/testing_files/read_file_header/NO_FILE.txt"
            ))
            .is_err()
        );
    }

    #[test]
    fn read_file_without_permissions() {
        assert!(
            read_file_header(&Path::new(
                "./tests/testing_files/read_file_header/no_permissions.txt"
            ))
            .is_err(),
        );
    }

    #[test]
    fn read_a_directory() {
        assert!(
            read_file_header(&Path::new("./tests/testing_files/read_file_header/dir")).is_err()
        );
    }

    #[test]
    fn read_binary_file() {
        assert_eq!(
            read_file_header(&Path::new(
                "./tests/testing_files/read_file_header/binary_file.exe"
            ))
            .unwrap(),
            vec![
                0x03, 0xD9, 0xA2, 0x9A, 0x67, 0xFB, 0x4B, 0xB5, 0x01, 0x00, 0x03, 0x00, 0x02, 0x10,
                0x00, 0x31, 0xC1, 0xF2, 0xE6, 0xBF, 0x71, 0x43, 0x50, 0xBE, 0x58, 0x05, 0x21, 0x6A,
                0xFC, 0x5A, 0xFF, 0x03, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x20, 0x00, 0x0E,
                0xEE, 0x76, 0x5F, 0x14, 0x0E, 0x18, 0xD5, 0x14, 0xD1, 0x89, 0xF7, 0x73, 0x2F, 0xC3,
                0x64, 0x9F, 0x99, 0xB3, 0xD7, 0x95, 0x47, 0x99
            ]
        )
    }

    #[test]
    fn read_binary_file2() {
        assert_eq!(
            read_file_header(&Path::new(
                "./tests/testing_files/read_file_header/binary_file_2.bin"
            ))
            .unwrap(),
            vec![
                0x03, 0xD9, 0xA2, 0x9A, 0x67, 0xFB, 0x4B, 0xB5, 0x00, 0x00, 0x04, 0x00, 0x02, 0x10,
                0x00, 0x00, 0x00, 0x31, 0xC1, 0xF2, 0xE6, 0xBF, 0x71, 0x43, 0x50, 0xBE, 0x58, 0x05,
                0x21, 0x6A, 0xFC, 0x5A, 0xFF, 0x03, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                0x04, 0x20, 0x00, 0x00, 0x00, 0xED, 0x1F, 0x69, 0x5B, 0x51, 0x1B, 0xE4, 0x7A, 0xFF,
                0xC0, 0xB7, 0xE6, 0x3A, 0x09, 0x72, 0x06, 0x59
            ]
        )
    }

    #[test]
    fn read_text_file() {
        assert_eq!(
            read_file_header(&Path::new(
                "./tests/testing_files/read_file_header/text_file.bib"
            ))
            .unwrap(),
            vec![
                0x40, 0x49, 0x6E, 0x50, 0x72, 0x6F, 0x63, 0x65, 0x65, 0x64, 0x69, 0x6E, 0x67, 0x73,
                0x7B, 0x31, 0x30, 0x2E, 0x31, 0x30, 0x30, 0x37, 0x2F, 0x39, 0x37, 0x38, 0x2D, 0x33,
                0x2D, 0x30, 0x33, 0x31, 0x2D, 0x37, 0x36, 0x32, 0x37, 0x33, 0x2D, 0x34, 0x5F, 0x38,
                0x2C, 0x0A, 0x61, 0x75, 0x74, 0x68, 0x6F, 0x72, 0x3D, 0x22, 0x4E, 0x67, 0x75, 0x79,
                0x65, 0x6E, 0x2C, 0x20, 0x54, 0x68, 0x61, 0x6E
            ]
        )
    }

    #[test]
    fn search_for_txt_extentions_val() {
        assert_eq!(
            HashSet::from_iter(
                file_search(
                    &Path::new("./tests/testing_files/file_searches/"),
                    &vec![String::from("txt")],
                    false
                )
                .unwrap()
                .iter()
                .cloned()
            ),
            HashSet::from([
                PathBuf::from("./tests/testing_files/file_searches/0/0.txt"),
                PathBuf::from("./tests/testing_files/file_searches/0/1.txt"),
                PathBuf::from("./tests/testing_files/file_searches/0/2.txt"),
                PathBuf::from("./tests/testing_files/file_searches/0/3.txt"),
                PathBuf::from("./tests/testing_files/file_searches/0/4.txt"),
                PathBuf::from("./tests/testing_files/file_searches/1/0.txt"),
                PathBuf::from("./tests/testing_files/file_searches/1/1.txt"),
                PathBuf::from("./tests/testing_files/file_searches/1/2.txt"),
                PathBuf::from("./tests/testing_files/file_searches/1/3.txt"),
                PathBuf::from("./tests/testing_files/file_searches/1/4.txt"),
                PathBuf::from("./tests/testing_files/file_searches/2/0.txt"),
                PathBuf::from("./tests/testing_files/file_searches/2/1.txt"),
                PathBuf::from("./tests/testing_files/file_searches/2/2.txt"),
                PathBuf::from("./tests/testing_files/file_searches/2/3.txt"),
                PathBuf::from("./tests/testing_files/file_searches/2/4.txt"),
                PathBuf::from("./tests/testing_files/file_searches/3/0.txt"),
                PathBuf::from("./tests/testing_files/file_searches/3/1.txt"),
                PathBuf::from("./tests/testing_files/file_searches/3/2.txt"),
                PathBuf::from("./tests/testing_files/file_searches/3/3.txt"),
                PathBuf::from("./tests/testing_files/file_searches/3/4.txt"),
                PathBuf::from("./tests/testing_files/file_searches/4/0.txt"),
                PathBuf::from("./tests/testing_files/file_searches/4/1.txt"),
                PathBuf::from("./tests/testing_files/file_searches/4/2.txt"),
                PathBuf::from("./tests/testing_files/file_searches/4/3.txt"),
                PathBuf::from("./tests/testing_files/file_searches/4/4.txt"),
            ])
        );
    }

    #[test]
    fn search_for_txt_extentions_cnt() {
        assert_eq!(
            file_search(
                &Path::new("./tests/testing_files/file_searches/"),
                &vec![String::from("txt")],
                false
            )
            .unwrap()
            .len(),
            25
        );
    }

    #[test]
    fn search_for_doc_extentions_val() {
        assert_eq!(
            HashSet::from_iter(
                file_search(
                    &Path::new("./tests/testing_files/file_searches/"),
                    &vec![String::from("doc")],
                    false
                )
                .unwrap()
                .iter()
                .cloned()
            ),
            HashSet::from([
                PathBuf::from("./tests/testing_files/file_searches/6/0.doc"),
                PathBuf::from("./tests/testing_files/file_searches/6/1.doc"),
                PathBuf::from("./tests/testing_files/file_searches/6/2.doc"),
                PathBuf::from("./tests/testing_files/file_searches/6/3.doc"),
                PathBuf::from("./tests/testing_files/file_searches/6/4.doc"),
            ])
        );
    }

    #[test]
    fn search_for_doc_extentions_cnt() {
        assert_eq!(
            file_search(
                &Path::new("./tests/testing_files/file_searches/"),
                &vec![String::from("doc")],
                false
            )
            .unwrap()
            .len(),
            5
        );
    }

    #[test]
    fn search_for_bin_extentions_val() {
        assert_eq!(
            HashSet::from_iter(
                file_search(
                    &Path::new("./tests/testing_files/file_searches/"),
                    &vec![String::from("bin")],
                    false
                )
                .unwrap()
                .iter()
                .cloned()
            ),
            HashSet::from([
                PathBuf::from("./tests/testing_files/file_searches/5/0.bin"),
                PathBuf::from("./tests/testing_files/file_searches/5/1.bin"),
                PathBuf::from("./tests/testing_files/file_searches/5/2.bin"),
                PathBuf::from("./tests/testing_files/file_searches/5/3.bin"),
                PathBuf::from("./tests/testing_files/file_searches/5/4.bin"),
            ])
        );
    }

    #[test]
    fn search_for_bin_extentions_cnt() {
        assert_eq!(
            file_search(
                &Path::new("./tests/testing_files/file_searches/"),
                &vec![String::from("bin")],
                false
            )
            .unwrap()
            .len(),
            5
        );
    }

    #[test]
    fn search_for_no_extentions_val() {
        assert_eq!(
            HashSet::from_iter(
                file_search(
                    &Path::new("./tests/testing_files/file_searches/"),
                    &vec![String::from("")],
                    false
                )
                .unwrap()
                .iter()
                .cloned()
            ),
            HashSet::from([
                PathBuf::from("./tests/testing_files/file_searches/7/0"),
                PathBuf::from("./tests/testing_files/file_searches/7/1"),
                PathBuf::from("./tests/testing_files/file_searches/7/2"),
                PathBuf::from("./tests/testing_files/file_searches/7/3"),
                PathBuf::from("./tests/testing_files/file_searches/7/4"),
            ])
        );
    }

    #[test]
    fn search_for_no_extentions_cnt() {
        assert_eq!(
            file_search(
                &Path::new("./tests/testing_files/file_searches/"),
                &vec![String::from("")],
                false
            )
            .unwrap()
            .len(),
            5
        );
    }

    #[test]
    fn search_for_all_extentions_val() {
        assert_eq!(
            HashSet::from_iter(
                file_search(
                    &Path::new("./tests/testing_files/file_searches/"),
                    &vec![
                        String::from("txt"),
                        String::from("bin"),
                        String::from("doc")
                    ],
                    false
                )
                .unwrap()
                .iter()
                .cloned()
            ),
            HashSet::from([
                PathBuf::from("./tests/testing_files/file_searches/0/0.txt"),
                PathBuf::from("./tests/testing_files/file_searches/0/1.txt"),
                PathBuf::from("./tests/testing_files/file_searches/0/2.txt"),
                PathBuf::from("./tests/testing_files/file_searches/0/3.txt"),
                PathBuf::from("./tests/testing_files/file_searches/0/4.txt"),
                PathBuf::from("./tests/testing_files/file_searches/1/0.txt"),
                PathBuf::from("./tests/testing_files/file_searches/1/1.txt"),
                PathBuf::from("./tests/testing_files/file_searches/1/2.txt"),
                PathBuf::from("./tests/testing_files/file_searches/1/3.txt"),
                PathBuf::from("./tests/testing_files/file_searches/1/4.txt"),
                PathBuf::from("./tests/testing_files/file_searches/2/0.txt"),
                PathBuf::from("./tests/testing_files/file_searches/2/1.txt"),
                PathBuf::from("./tests/testing_files/file_searches/2/2.txt"),
                PathBuf::from("./tests/testing_files/file_searches/2/3.txt"),
                PathBuf::from("./tests/testing_files/file_searches/2/4.txt"),
                PathBuf::from("./tests/testing_files/file_searches/3/0.txt"),
                PathBuf::from("./tests/testing_files/file_searches/3/1.txt"),
                PathBuf::from("./tests/testing_files/file_searches/3/2.txt"),
                PathBuf::from("./tests/testing_files/file_searches/3/3.txt"),
                PathBuf::from("./tests/testing_files/file_searches/3/4.txt"),
                PathBuf::from("./tests/testing_files/file_searches/4/0.txt"),
                PathBuf::from("./tests/testing_files/file_searches/4/1.txt"),
                PathBuf::from("./tests/testing_files/file_searches/4/2.txt"),
                PathBuf::from("./tests/testing_files/file_searches/4/3.txt"),
                PathBuf::from("./tests/testing_files/file_searches/4/4.txt"),
                PathBuf::from("./tests/testing_files/file_searches/6/0.doc"),
                PathBuf::from("./tests/testing_files/file_searches/6/1.doc"),
                PathBuf::from("./tests/testing_files/file_searches/6/2.doc"),
                PathBuf::from("./tests/testing_files/file_searches/6/3.doc"),
                PathBuf::from("./tests/testing_files/file_searches/6/4.doc"),
                PathBuf::from("./tests/testing_files/file_searches/5/0.bin"),
                PathBuf::from("./tests/testing_files/file_searches/5/1.bin"),
                PathBuf::from("./tests/testing_files/file_searches/5/2.bin"),
                PathBuf::from("./tests/testing_files/file_searches/5/3.bin"),
                PathBuf::from("./tests/testing_files/file_searches/5/4.bin"),
            ])
        );
    }

    #[test]
    fn search_for_all_extentions_cnt() {
        assert_eq!(
            file_search(
                &Path::new("./tests/testing_files/file_searches/"),
                &vec![
                    String::from("txt"),
                    String::from("bin"),
                    String::from("doc")
                ],
                false
            )
            .unwrap()
            .len(),
            35
        );
    }

    #[test]
    #[should_panic]
    fn extention_seach_path_is_file() {
        file_search(
            &Path::new("./tests/testing_files/file_searches/0/0.txt"),
            &vec![
                String::from("txt"),
                String::from("bin"),
                String::from("doc"),
            ],
            false,
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn extention_seach_path_does_not_exist() {
        file_search(
            &Path::new("./tests/testing_files/file_searches/FOLDER"),
            &vec![
                String::from("txt"),
                String::from("bin"),
                String::from("doc"),
            ],
            false,
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    #[cfg(target_os = "linux")]
    fn linux_forbidden_directory_access() {
        file_search(
            &Path::new("/boot/efi/EFI/"),
            &vec![
                String::from("txt"),
                String::from("bin"),
                String::from("doc"),
            ],
            false,
        )
        .unwrap();
    }

    #[test]
    fn txt_file_search_dir_0_cnt() {
        assert_eq!(
            file_search(
                &Path::new("./tests/testing_files/file_searches/0"),
                &Vec::new(),
                true
            )
            .unwrap()
            .len(),
            3
        );
    }

    #[test]
    fn txt_file_search_dir_1_cnt() {
        assert_eq!(
            file_search(
                &Path::new("./tests/testing_files/file_searches/1"),
                &Vec::new(),
                true
            )
            .unwrap()
            .len(),
            3
        );
    }

    #[test]
    fn txt_file_search_dir_2_cnt() {
        assert_eq!(
            file_search(
                &Path::new("./tests/testing_files/file_searches/2"),
                &Vec::new(),
                true
            )
            .unwrap()
            .len(),
            3
        );
    }

    #[test]
    fn txt_file_search_dir_3_cnt() {
        assert_eq!(
            file_search(
                &Path::new("./tests/testing_files/file_searches/3"),
                &Vec::new(),
                true
            )
            .unwrap()
            .len(),
            3
        );
    }

    #[test]
    fn txt_file_search_dir_4_cnt() {
        assert_eq!(
            file_search(
                &Path::new("./tests/testing_files/file_searches/4"),
                &Vec::new(),
                true
            )
            .unwrap()
            .len(),
            3
        );
    }

    #[test]
    fn txt_file_search_dir_5_cnt() {
        assert_eq!(
            file_search(
                &Path::new("./tests/testing_files/file_searches/5"),
                &Vec::new(),
                true
            )
            .unwrap()
            .len(),
            3
        );
    }

    #[test]
    fn txt_file_search_dir_6_cnt() {
        assert_eq!(
            file_search(
                &Path::new("./tests/testing_files/file_searches/6"),
                &Vec::new(),
                true
            )
            .unwrap()
            .len(),
            3
        );
    }

    #[test]
    fn txt_file_search_dir_7_cnt() {
        assert_eq!(
            file_search(
                &Path::new("./tests/testing_files/file_searches/7"),
                &Vec::new(),
                true
            )
            .unwrap()
            .len(),
            4
        );
    }

    #[test]
    fn txt_file_search_dir_0_val() {
        assert_eq!(
            HashSet::from_iter(
                file_search(
                    &Path::new("./tests/testing_files/file_searches/0"),
                    &Vec::new(),
                    true
                )
                .unwrap()
                .iter()
                .cloned()
            ),
            HashSet::from([
                PathBuf::from("./tests/testing_files/file_searches/0/0.txt"),
                PathBuf::from("./tests/testing_files/file_searches/0/1.txt"),
                PathBuf::from("./tests/testing_files/file_searches/0/4.txt")
            ])
        );
    }

    #[test]
    fn txt_file_search_dir_1_val() {
        assert_eq!(
            HashSet::from_iter(
                file_search(
                    &Path::new("./tests/testing_files/file_searches/1"),
                    &Vec::new(),
                    true
                )
                .unwrap()
                .iter()
                .cloned()
            ),
            HashSet::from([
                PathBuf::from("./tests/testing_files/file_searches/1/2.txt"),
                PathBuf::from("./tests/testing_files/file_searches/1/3.txt"),
                PathBuf::from("./tests/testing_files/file_searches/1/4.txt")
            ])
        );
    }

    #[test]
    fn txt_file_search_dir_2_val() {
        assert_eq!(
            HashSet::from_iter(
                file_search(
                    &Path::new("./tests/testing_files/file_searches/2"),
                    &Vec::new(),
                    true
                )
                .unwrap()
                .iter()
                .cloned()
            ),
            HashSet::from([
                PathBuf::from("./tests/testing_files/file_searches/2/2.txt"),
                PathBuf::from("./tests/testing_files/file_searches/2/3.txt"),
                PathBuf::from("./tests/testing_files/file_searches/2/4.txt")
            ])
        );
    }

    #[test]
    fn txt_file_search_dir_3_val() {
        assert_eq!(
            HashSet::from_iter(
                file_search(
                    &Path::new("./tests/testing_files/file_searches/3"),
                    &Vec::new(),
                    true
                )
                .unwrap()
                .iter()
                .cloned()
            ),
            HashSet::from([
                PathBuf::from("./tests/testing_files/file_searches/3/1.txt"),
                PathBuf::from("./tests/testing_files/file_searches/3/2.txt"),
                PathBuf::from("./tests/testing_files/file_searches/3/3.txt"),
            ])
        );
    }

    #[test]
    fn txt_file_search_dir_4_val() {
        assert_eq!(
            HashSet::from_iter(
                file_search(
                    &Path::new("./tests/testing_files/file_searches/4"),
                    &Vec::new(),
                    true
                )
                .unwrap()
                .iter()
                .cloned()
            ),
            HashSet::from([
                PathBuf::from("./tests/testing_files/file_searches/4/0.txt"),
                PathBuf::from("./tests/testing_files/file_searches/4/3.txt"),
                PathBuf::from("./tests/testing_files/file_searches/4/4.txt")
            ])
        );
    }

    #[test]
    fn txt_file_search_dir_5_val() {
        assert_eq!(
            HashSet::from_iter(
                file_search(
                    &Path::new("./tests/testing_files/file_searches/5"),
                    &Vec::new(),
                    true
                )
                .unwrap()
                .iter()
                .cloned()
            ),
            HashSet::from([
                PathBuf::from("./tests/testing_files/file_searches/5/0.bin"),
                PathBuf::from("./tests/testing_files/file_searches/5/1.bin"),
                PathBuf::from("./tests/testing_files/file_searches/5/4.bin")
            ])
        );
    }

    #[test]
    fn txt_file_search_dir_6_val() {
        assert_eq!(
            HashSet::from_iter(
                file_search(
                    &Path::new("./tests/testing_files/file_searches/6"),
                    &Vec::new(),
                    true
                )
                .unwrap()
                .iter()
                .cloned()
            ),
            HashSet::from([
                PathBuf::from("./tests/testing_files/file_searches/6/0.doc"),
                PathBuf::from("./tests/testing_files/file_searches/6/1.doc"),
                PathBuf::from("./tests/testing_files/file_searches/6/3.doc"),
            ])
        );
    }

    #[test]
    fn txt_file_search_dir_7_val() {
        assert_eq!(
            HashSet::from_iter(
                file_search(
                    &Path::new("./tests/testing_files/file_searches/7"),
                    &Vec::new(),
                    true
                )
                .unwrap()
                .iter()
                .cloned()
            ),
            HashSet::from([
                PathBuf::from("./tests/testing_files/file_searches/7/0"),
                PathBuf::from("./tests/testing_files/file_searches/7/1"),
                PathBuf::from("./tests/testing_files/file_searches/7/3"),
                PathBuf::from("./tests/testing_files/file_searches/7/4")
            ])
        );
    }
}
