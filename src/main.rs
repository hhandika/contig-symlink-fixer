use std::fs;
use std::os::unix;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

fn main() {
    let path = ".";
    println!("Fixing symplink");
    find_contigs(path);
    println!("DONE!");
}

fn find_contigs(path: &str) {
    let symdir = Path::new("contig_symlink_fix");
    fs::create_dir_all(symdir).expect("CAN'T CREATE SYMLINK FOLDER.");

    WalkDir::new(path)
        .into_iter()
        .filter_map(|ok| ok.ok())
        .filter(|e| e.file_type().is_file())
        .for_each(|e| {
            let fname = String::from(e.path().to_string_lossy());
            if fname.ends_with("/contigs.fasta") {
                let path = PathBuf::from(fname);

                // Get ID from path to the contigs. Assuming the first index is the species ID
                let dirs: Vec<_> = path.components().map(|d| d.as_os_str()).collect();
                assert!(dirs.len() > 1, "INVALID FOLDER STRUCTURE TO USE AUTO");
                let id = String::from(dirs[1].to_string_lossy());
                let sym_name = format!("{}_contigs.fasta", id);
                let contig_sym = symdir.join(sym_name);
                create_symlink(&path, &contig_sym);
            }
        });
}

fn create_symlink(path: &Path, contig_sym: &Path) {
    let sympath = path.canonicalize().expect("CAN'T CANOCALIZE PATH");
    unix::fs::symlink(&sympath, contig_sym).expect("CAN'T CREATE SYMLINK");
    println!(
        "{} -> {}",
        sympath.to_string_lossy(),
        contig_sym.to_string_lossy()
    );
}
