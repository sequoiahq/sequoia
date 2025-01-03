## Levels

### SL150
- **Description**: Development certificate.
- **Capabilities**: 
  - Useful for developing a PlayReady client but generally considered useless for streaming services.
- **Limitations**: 
  - Likely restricted to SD resolution.

### SL2000
- **Description**: Basic security for HD content.
- **Capabilities**: 
  - Supports device authentication and licensing mechanisms.
  - Generally provides up to 1080p on Netflix, Disney+, and Amazon; 2160p on MAX and some devices *may* reach 2160p on AMZN.

### SL3000
- **Description**: Enhanced security features compared to SL2000.
- **Capabilities**: 
  - Better suited for higher resolution content (e.g., 2160p).
  - Provides a secure environment without the risk of CDM revocation.

### Client Authentication

PlayReady devices rely on two primary files for client authentication.

- **`bgroupcert.dat`**: Group certificate issued by Microsoft specifically to a device.[1]
- **`zgpriv.dat`**: A 32-byte ECC private signing key.

The group certificate alone is not usable and requires to be fitted with ECC encryption and a signing key.

To integrate these into the certificate chain:
1. A new certificate is created containing the respective ECC public keys (encryption/signing).
2. This certificate is signed with the group private key.
3. The signature includes the group public key, enabling server-side signature verification.

The resulting activated certificate is known as `bdevcert.dat`.

---

### PSSH (Protection System Specific Header)

A PlayReady PSSH comprises a PlayReady Object, which contains one or more PlayReady Object Records. Each record is identified by a type value. Currently, only type `1` is functional, corresponding to the PlayReady Header. This header contains XML encoded in UTF-16-LE, known as the WRM Header.

---

### License Request

Client-server communication in PlayReady operates entirely through XML.

- The WRM Header is embedded in the `ContentHeader` section of the license request.
- To secure the communication, the WMRMServer public key encrypts the keys, ensuring only the server can read them.

The encryption process involves the following steps:
1. Generate a temporary ECC key for the request.
2. Derive an AES key and IV by splitting the X coordinate of the ECC key into two halves.
3. Encrypt the ECC key's X and Y coordinates using the WMRMServer public key via the ElGamal encryption algorithm. This produces a 128-byte `EncryptedKey`.
4. Use the AES key and IV to encrypt the activated group certificate, with the IV prepended to the ciphertext.
5. Create a SHA-256 digest from the `LA` section of the challenge and include it in the `SignedInfo` element of the XML.
6. Sign this digest using the temporary encryption key (SHA-256, FIPS 186-3), generating the `SignatureValue`.

The temporary signing key is also included in the challenge to allow the server to verify the signature.

---

### License

Internally referred to as the XMR License, this license contains ElGamal-encrypted content keys.

The structure includes containers defined by flags, tags, and lengths. A type value of `10` represents a ContentKey Object, capable of holding multiple encrypted content keys. Each key consists of two ECC points, each with a 32-byte X and Y coordinate (128 bytes total). 

To decrypt the content keys:
1. Use the previously generated encryption key to decrypt the ECC points.
2. Combine the X and Y coordinates into a single ECC point.
3. Extract the X coordinate, convert it to bytes, and use the latter 16 bytes to derive the content key.


### Notes
[1] In PlayReady, *some* devices share the same group certificate. I'm gonna talk about that in another commit but I want to acclare that if a
    specific platform (e.g Amazon, Max) bans a specific manipulated provision, **it will only affect that provision**, not the device itself. That makes the possibility of *reprovisioning* devices (hell, even pyPlayready implemented that) because the new provision will not be blacklisted by the provider. Note that these certificates burn ~24h after provisioning that device, but you can reprovision the same group certificate as many times you want. Until now I didn't see any provider banning **the group certificate itself**, just Amazon unwhitelisting the provision in-house.