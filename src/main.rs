//mod header;
mod messagetype;
mod processingtype;
mod versionidentifier;
mod codedelement;
//mod id;

use crate::versionidentifier::VersionIdentifier;



fn test_processing_type() {
    let pt_valid = VersionIdentifier::parse("2.5");
    assert!(pt_valid.is_some());

    let pt_valid_s = VersionIdentifier::parse("2.1");
    assert!(pt_valid_s.is_some());
}

fn main() {
    test_processing_type();
    println!("ProcessingType tests passed");
}
