use std::fs;
use std::os::unix;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub fn generate_symlink_all(path: &str) -> u32 {
    let symdir = Path::new("contig_symlink");
    fs::create_dir_all(symdir).expect("CAN'T CREATE SYMLINK FOLDER.");
    let mut counts = 0;

    WalkDir::new(path)
        .into_iter()
        .filter_map(|ok| ok.ok())
        .filter(|e| e.file_type().is_file())
        .for_each(|e| {
            let fname = String::from(e.path().to_string_lossy());
            if fname.ends_with("/contigs.fasta") {
                let path = PathBuf::from(fname);
                create_symlink(&path, &symdir);
                counts += 1;
            }
        });
    counts
}

fn create_symlink(path: &Path, symdir: &Path) {
    // Use folder name as contig name.
    let fname = get_fname(path);
    let contig_sym = symdir.join(fname);
    let sympath = path.canonicalize().expect("CAN'T CANOCALIZE PATH");

    unix::fs::symlink(&sympath, &contig_sym).expect("CAN'T CREATE SYMLINK");
    println!(
        "{} \x1b[0;45m => \x1b[0m {}",
        sympath.to_string_lossy(),
        contig_sym.to_string_lossy()
    );
}

fn get_fname(path: &Path) -> PathBuf {
    let dirs: Vec<_> = path.components().map(|d| d.as_os_str()).collect();
    assert!(dirs.len() > 1, "INVALID FOLDER STRUCTURE");
    let mut fname = PathBuf::from(String::from(dirs[1].to_string_lossy()));
    fname.set_extension(path.extension().unwrap());
    fname
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn output_fname_test() {
        let path = Path::new("./Species_epithet_ABCD123425/contigs.fasta");
        let res = PathBuf::from("Species_epithet_ABCD123425.fasta");

        assert_eq!(res, get_fname(path));
    }
}
