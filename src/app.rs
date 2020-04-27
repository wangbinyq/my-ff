use clap::{clap_app, ArgMatches, crate_version, crate_authors };

const ABOUT: &str = "
Find Files (ff) utility recursively searches the files whose names match the
specified RegExp pattern in the provided directory (defaults to the current
directory if not provided).";

pub fn app() -> ArgMatches<'static> {
    lazy_static! {
        static ref WORKING_DIR_PATH: String = working_dir_path();
    }


    clap_app!(ff =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: ABOUT)
        (max_term_width: 80)
        (@arg PATTERN: +required "Find files whoes name (path) matches this substring or the regular expression.")
        (@arg ROOT_PATH: default_value(&WORKING_DIR_PATH) "Path to the directory to search files inside.")
        (@arg exclude_directories: -D --("exclude-dir-paths") "Exclude paths from the search result which are directories and not files.")
        (@arg ignore_hidden: -H --("ignore-hidden") "Ignore searching hidden files and directories. By default, hidden files and directories are included in the search results.")
        (@arg ignore_gitignore: -G --("ignore-gitignore") "Ignore searching files and directories specified in .gitignore. By default, the files and directories specified in .gitignore are included in the search results.")
        (@arg case_sensitive: -s --("case-sensitive") "Search case sensitively. By default, files are searched case insensitively.")
        (@arg level: -L --level +takes_value "Recursively search only given level directories deep. By default no depth restriction is imposed. A value of 0 would always yield zero results. A value of 1 searches for the direct children in the given path.")
        (@arg threads: -j --threads +takes_value "The approximate number of threads to use. A value of 0 (which is the default) results in thread count set to available CPU cores.")
        (@arg exclude: -x --exclude +takes_value "Exclude files and directories matching this regular expression from the search results.")
    ).get_matches()
}

pub fn working_dir_path() -> String {
    match std::env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(_) => ".".to_string()
    }
}