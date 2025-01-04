use std::error::Error;
use std::io::Read;
use std::net::TcpStream;

use cdm::Cdm;
use device::Device;
use pssh::PSSH;
use xmrlicense::XmrLicense;

fn main() -> Result<(), Box<dyn Error>> {
    // load device
    let device = Device::load("device.prd")?;

    // create cdm instance from the device
    let cdm = Cdm::from_device(device)?;

    // open session
    let session_id = cdm.open()?;

    // define pssh
    let pssh = PSSH::new(
        "AAADfHBzc2gAAAAAmgTweZhAQoarkuZb4IhflQAAA1xcAwAAAQABAFIDPABXAFIATQBIAEUAQQBEAEUAUgAgAHgAbQBsAG4AcwA9ACIAaAB0AHQAcAA6AC8ALwBzAGMAaABlAG0AYQBzAC4AbQBpAGMAcgBvAHMAbwBmAHQALgBjAG8AbQAvAEQAUgBNAC8AMgAwADAANwAvADAAMwAvAFAAbABhAHkAUgBlAGEAZAB5AEgAZQBhAGQAZQByACIAIAB2AGUAcgBzAGkAbwBuAD0AIgA0AC4AMAAuADAALgAwACIAPgA8AEQAQQBUAEEAPgA8AFAAUgBPAFQARQBDAFQASQBOAEYATwA+ADwASwBFAFkATABFAE4APgAxADYAPAAvAEsARQBZAEwARQBOAD4APABBAEwARwBJAEQAPgBBAEUAUwBDAFQAUgA8AC8AQQBMAEcASQBEAD4APAAvAFAAUgBPAFQARQBDAFQASQBOAEYATwA+ADwASwBJAEQAPgA0AFIAcABsAGIAKwBUAGIATgBFAFMAOAB0AEcAawBOAEYAVwBUAEUASABBAD0APQA8AC8ASwBJAEQAPgA8AEMASABFAEMASwBTAFUATQA+AEsATABqADMAUQB6AFEAUAAvAE4AQQA9ADwALwBD"
    );

    // get wrm headers
    let wrm_headers = pssh.get_wrm_headers(false);

    // create a license challenge
    let license_challenge = cdm.get_license_challenge(session_id, wrm_headers[0].clone())?;

    // send challenge to server
    let mut stream = TcpStream::connect("test.playready.microsoft.com:80")?;
    let request = format!("POST /service/rightsmanager.asmx?cfg=(persist:false,sl:2000) HTTP/1.1\r\nHost: test.playready.microsoft.com\r\nContent-Type: text/xml; charset=UTF-8\r\n\r\n{}", license_challenge);
    stream.write_all(request.as_bytes())?;

    // read response
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // parse response
    let license = XmrLicense::loads(response.as_bytes())?;

    // get keys
    let content_keys = license.get_content_keys().collect::<Vec<_>>();

    // print keys
    for key in content_keys {
        println!("{}: {}", key.key_id, key.key);
    }

    // close session
    cdm.close(session_id)?;

    Ok(())
}
