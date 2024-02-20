use sp_core::Decode;
use subxt_metadata::Metadata;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = include_str!("./metadata.txt");
    let hexstr = metadata
        .trim_matches('\"')
        .to_string()
        .trim_start_matches("0x")
        .to_string();

    let decoded = hex::decode(hexstr)?;
    let metadata: Metadata = Decode::decode(&mut decoded.as_slice())?;
    Ok(())
}
