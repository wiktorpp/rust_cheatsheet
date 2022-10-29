#[derive(Debug)]
enum Version { NoVersion, Version1, Version2 }

fn parse_version(version: u8) -> Result<Version, &'static str> {
    if version == 1 {
        return Ok(Version::Version1);
    } else if version == 2 {
        return Ok(Version::Version2)
    } else {
        return Err("invalid version")
    }
}

fn main() {
    let version = parse_version(3);
    match version {
        Ok(v) => println!("working with version: {v:?}"),
        Err(e) => println!("error parsing header: {e:?}"),
    }

    // this will panic
    println!("{:?}", parse_version(3).unwrap())
}