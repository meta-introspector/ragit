use crate::prelude::*;
use crate::index::commands::archive::create::init_workers;
use std::thread;
use std::time::Duration;
use regex::Regex;
use ragit_fs::{exists, remove_file, read_dir, file_name};
use crate::index::commands::archive::create::request::Request;

impl Index {
    pub fn create_archive(
        &self,
        workers: usize,
        size_limit: Option<u64>,
        output: String,
        include_configs: bool,
        include_prompts: bool,
        force: bool,
        quiet: bool,
    ) -> Result<(), Error> {
        if self.curr_processing_file.is_some() {
            return Err(Error::DirtyKnowledgeBase);
        }

        let workers = init_workers(
            workers,
            &self.root_dir,
            6,
        );

        let real_output = if size_limit.is_some() {
            format!("{output}-0000")
        } else {
            output.clone()
        };
        let already_exists = exists(&real_output);

        if already_exists && !force {
            return Err(FileError {
                kind: FileErrorKind::AlreadyExists,
                given_path: Some(real_output),
            }.into());
        }

        match self.create_archive_worker(
            &workers,
            size_limit,
            output.clone(),
            include_configs,
            include_prompts,
            quiet,
        ) {
            Ok(()) => Ok(()),
            Err(e) => {
                let tmp_file_name_re = Regex::new(r"__archive_block_\d{6}_\d{6}").unwrap();

                for worker in workers.iter() {
                    let _ = worker.send(Request::Kill);
                }

                thread::sleep(Duration::from_millis(500));

                if size_limit.is_some() {
                    for i in 0..10000 {
                        let output_file = format!("{output}-{i:06}");
                        if exists(&output_file) {
                            let _ = remove_file(&output_file);
                        } else {
                            break;
                        }
                    }
                } else if exists(&output) {
                    let _ = remove_file(&output);
                }

                for file in read_dir(".", false)? {
                    if let Ok(file) = file_name(&file) {
                        if tmp_file_name_re.is_match(&file) {
                            let _ = remove_file(&file);
                        }
                    }
                }

                Err(e)
            },
        }
    }
}
