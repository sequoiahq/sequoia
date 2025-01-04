## Widevine Levels

### L1
- **Description**: Highest level of security.
- **Capabilities**: 
  - Allows decryption of content in a Trusted Execution Environment (TEE).
  - No limitations on playback resolution or features (e.g DoVi or Atmos).
- **Cautions**: 
  - Highly controlled; strange requests to license servers ~~can lead to the revocation of the CDM~~ are gonna revoke the CDM.
  - Release groups no longer utilize L1 due to the need for vulnerable devices. Using PlayReady provide the exact same results at a much lower cost.

### L2
- **Description**: Mixed security environment. Is not used for mobile devices. 
- **Capabilities**: 
  - Media decryption and processing occurs in software or dedicated video hardware, despite the presence of a TEE, and content is available in a fixed resolution
- **Cautions**: 
  - Rarely used for decryption; obtaining L2 is often considered impractical.

### L3
- **Sublevels**: L3 can be (simplified) divided into Android L3 and Chrome L3.

#### Android L3
- **Description**: This is the lowest level of security. It was (publicly) cracked by David Buchanan, who highlighted a vulnerability in the L3 Whitebox AES-128 implementation on X, which can be found in [this tweet](https://x.com/David3141593/status/1080606827384131590). 
The issue comes from an exploit that allowed to use special inputs to cra

The issue comes from an exploit that allows attackers to use specially crafted inputs to slow down or crash applications by overwhelming their systems with inefficient patterns.

- **Capabilities**: 
  - Easy to obtain; can give up to 480p resolution on services like Amazon, Netflix.
- **Cautions**: 
  - Vulnerable by design and primarily used for SD content. Not recommended for major streaming services due to limited security.

#### Chrome L3
- **Description**: Slightly better than Android L3.
- **Capabilities**: 
  - Supports up to 1080p on Netflix and Amazon, and 720p on Disney+ and Max.
- **Cautions**: 
  - Extremely hard to obtain than Android L3 but does not get revoked (the certificate is the same on all instances). 
  - Integrated into the Chrome browser, requiring frequent reverse engineering with each browser update.
