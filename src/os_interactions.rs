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

/// Determine the count of the ascii characters within a file and
/// return a vector with the counts of each character.
fn file_char_cnt(file: &Path) -> Result<Vec<usize>, Box<dyn Error>> {
    return Ok(Vec::new());
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
    #[should_panic]
    fn read_non_existant_file() {
        read_file_header(&Path::new(
            "./tests/testing_files/read_file_header/NO_FILE.txt",
        ))
        .unwrap();
    }
    #[test]
    #[should_panic]
    fn read_file_without_permissions() {
        read_file_header(&Path::new(
            "./tests/testing_files/read_file_header/no_permissions.txt",
        ))
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn read_a_directory() {
        read_file_header(&Path::new("./tests/testing_files/read_file_header/dir")).unwrap();
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

    #[test]
    #[should_panic]
    fn char_cnt_file_not_exist() {
        let _ = file_char_cnt(&Path::new(
            "./tests/testing_files/file_char_freq/DOES_NOT_EXIST",
        ));
    }

    #[test]
    #[should_panic]
    #[cfg(target_os = "linux")]
    fn char_cnt_file_not_readable_linux() {
        let _ = file_char_cnt(&Path::new("/etc/shadow"));
    }

    #[test]
    #[should_panic]
    fn char_cnt_open_folder() {
        let _ = file_char_cnt(&Path::new("./tests/testing_files/file_char_freq"));
    }

    #[test]
    #[should_panic]
    #[cfg(target_os = "linux")]
    fn char_cnt_file_not_reachable_linux() {
        let _ = file_char_cnt(&Path::new("/boot/efi/EFI/BOOT/BOOTX64.EFI"));
    }

    #[test]
    #[should_panic]
    #[cfg(target_os = "windows")]
    fn char_cnt_file_not_reachable_linux() {
        let _ = file_char_cnt(&Path::new(r"C:\Windows\System32\Config\SAM"));
    }

    #[test]
    fn char_cnt_empty_file() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/empty_file.txt"
            ))
            .unwrap(),
            vec![0; 128]
        );
    }

    #[test]
    fn char_cnt_ascii_file_0() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/ascii_file_0.txt"
            ))
            .unwrap(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 172, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 1, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 6, 2, 1, 2, 2, 1, 2, 1, 6, 0, 0, 0, 3, 0, 5, 1, 0,
                2, 5, 4, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 10, 8, 37, 94, 24, 19, 57, 54, 1,
                5, 28, 17, 58, 67, 16, 0, 52, 57, 99, 25, 8, 14, 0, 12, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn char_cnt_ascii_file_1() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/ascii_file_1.txt"
            ))
            .unwrap(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 377, 0, 0, 0, 0, 0, 0, 1, 2, 2, 0, 0, 45, 7, 18, 0, 18, 2, 1, 1, 1,
                0, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 11, 1, 0, 7, 3, 0, 0, 0, 1, 0, 0,
                0, 0, 1, 5, 7, 1, 1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 133, 16, 46, 61, 228, 38, 21,
                120, 109, 4, 10, 56, 34, 141, 142, 28, 1, 119, 138, 187, 57, 22, 22, 3, 27, 3, 0,
                0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn char_cnt_ascii_file_2() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/ascii_file_2.txt"
            ))
            .unwrap(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 202, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 7, 20, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 11, 6, 2, 1, 4, 0, 6, 1, 7, 0, 2, 5, 2, 2, 2,
                3, 0, 3, 2, 5, 2, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 73, 12, 21, 30, 108, 26, 11, 40,
                52, 1, 10, 34, 22, 71, 67, 17, 1, 46, 43, 78, 22, 9, 20, 3, 15, 1, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn char_cnt_ascii_file_3() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/ascii_file_3.txt"
            ))
            .unwrap(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 135, 0, 0, 0, 0, 0, 2, 0, 8, 8, 11, 0, 2, 3, 2, 10, 2, 3, 5, 1, 0,
                4, 0, 1, 0, 1, 0, 11, 0, 8, 2, 1, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 14, 5, 4, 3, 19, 12, 4, 10, 15,
                0, 2, 14, 4, 14, 13, 1, 1, 13, 7, 22, 5, 3, 1, 5, 13, 0, 1, 0, 1, 0, 0
            ]
        );
    }

    #[test]
    fn char_cnt_ascii_file_4() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/ascii_file_4.txt"
            ))
            .unwrap(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 9, 0, 0, 3, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 3, 6, 5, 1, 0, 4, 8, 0, 1, 5, 1, 6, 4,
                0, 0, 2, 3, 5, 5, 0, 1, 0, 0, 0, 2, 0, 2, 0, 0
            ]
        );
    }

    #[test]
    fn char_cnt_utf8_file_0() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/utf8_file_0.txt"
            ))
            .unwrap(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 346, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 3167, 13, 0, 0, 0, 0, 0, 0, 1, 1, 5, 0, 224, 21, 172, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 20, 24, 0, 0, 0, 8, 0, 23, 20, 18, 10, 4, 11, 6, 29, 142, 0, 2,
                4, 16, 3, 8, 4, 1, 21, 15, 41, 3, 0, 8, 0, 2, 0, 0, 0, 0, 0, 10, 0, 1136, 173, 393,
                635, 1812, 329, 340, 945, 790, 5, 97, 586, 347, 989, 1010, 196, 8, 791, 872, 1244,
                381, 136, 324, 14, 247, 6, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn char_cnt_utf8_file_1() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/utf8_file_1.txt"
            ))
            .unwrap(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 92, 1, 0, 0, 0, 0, 0, 1, 2, 2, 0, 0, 19, 0, 40, 2, 1, 0, 0, 0, 0, 1,
                0, 1, 0, 0, 0, 0, 9, 0, 9, 0, 17, 9, 9, 14, 5, 5, 2, 6, 1, 6, 5, 2, 3, 6, 7, 2, 6,
                0, 5, 14, 2, 8, 2, 9, 0, 2, 0, 0, 0, 0, 0, 0, 0, 72, 13, 43, 36, 120, 9, 16, 18,
                85, 2, 10, 53, 35, 69, 51, 16, 0, 59, 54, 50, 27, 23, 6, 2, 15, 4, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn char_cnt_utf8_file_2() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/utf8_file_2.txt"
            ))
            .unwrap(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 290, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn char_cnt_utf8_file_3() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/utf8_file_3.txt"
            ))
            .unwrap(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 283, 8, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 38, 1, 9, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 2, 6, 0, 0, 0, 0, 0, 11, 0, 0, 3, 7, 2, 1, 2, 0, 1, 4, 2, 4, 2, 0, 4,
                0, 1, 8, 4, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 179, 11, 1, 49, 120, 14, 16, 2,
                137, 46, 41, 94, 58, 129, 116, 22, 0, 80, 75, 56, 33, 41, 0, 0, 0, 4, 0, 0, 0, 0,
                0
            ]
        );
    }

    #[test]
    fn char_cnt_utf8_file_4() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/utf8_file_4.txt"
            ))
            .unwrap(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 319, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 1144, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 103, 5, 83, 0, 0, 3, 0, 0, 2,
                0, 1, 0, 0, 0, 2, 24, 0, 0, 0, 3, 0, 66, 14, 25, 46, 24, 22, 32, 14, 79, 1, 0, 32,
                18, 29, 27, 6, 0, 32, 15, 5, 15, 15, 13, 33, 34, 0, 2, 0, 2, 0, 0, 0, 213, 55, 91,
                197, 162, 77, 68, 80, 190, 0, 0, 101, 50, 207, 119, 11, 0, 202, 35, 59, 61, 0, 84,
                0, 129, 0, 3, 0, 3, 0, 0
            ]
        );
    }

    #[test]
    fn char_cnt_utf16_file_0() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/utf16_file_0.txt"
            ))
            .unwrap(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 80, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 157, 161, 25, 29,
                23, 23, 39, 7, 5, 37, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 17, 1, 1, 1, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn char_cnt_utf16_file_1() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/utf16_file_1.txt"
            ))
            .unwrap(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 138, 120, 3, 7,
                21, 27, 17, 28, 15, 3, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 13, 1, 1, 1, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn char_cnt_utf16_file_2() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/utf16_file_2.txt"
            ))
            .unwrap(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 156, 156, 13, 13,
                6, 24, 40, 41, 5, 19, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 16, 1, 1, 1, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn char_cnt_utf16_file_3() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/utf16_file_3.txt"
            ))
            .unwrap(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 60, 0, 0, 3, 7,
                11, 19, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 1, 1, 1, 5, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn char_cnt_utf16_file_4() {
        assert_eq!(
            file_char_cnt(&Path::new(
                "./tests/testing_files/file_char_freq/utf16_file_4.txt"
            ))
            .unwrap(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 3, 2, 2, 2, 2,
                2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 1, 3, 0, 1, 0, 0,
                0, 2, 2, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }
}
