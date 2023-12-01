use dicom::dictionary_std::tags;
use dicom::object::open_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let obj = open_file("0002.DCM")?;
    let patient_name = obj.element(tags::PATIENT_NAME)?.to_str()?;
    println!("Patient {:?}", patient_name);
    Ok(())
}
