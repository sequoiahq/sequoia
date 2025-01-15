The Sequoia MetaContainer is a JSON structure for storing information about the CDM. 
#### Structure

```json
{
  "MetaContainer": {
    "userAgent": "string",
    "systemId": "string",
    "securityLevel": "string",
    "manufacturer": "string",
    "model": "string"
  }
}
```

#### Fields

- **userAgent**: Useragent string
  - *Example*: `"Mozilla/5.0 (Linux; Android 11; Pixel 5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Mobile Safari/537.36"`

- **systemId**: The identifier for the Widevine DRM system.
  - *Type*: String
  - *Example*: `"8159"`

- **securityLevel**:
  - *Type*: String
  - *Example*: `"L3"`

- **manufacturer**: The name of the device manufacturer.
  - *Type*: String
  - *Example*: `"Google"`

- **model**: The model of the device.
  - *Type*: String
  - *Example*: `"Pixel 5"`

### Example
An example of a MetaContainer file for a Sharp Aquos R6 phone (example taken from a leaked CDM). 

```json
{
  "MetaContainer": {
    "userAgent": "Mozilla/5.0 (Linux; U; Android 13; SH-51B Build/S7033; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/131.0.6778.135 Mobile Safari/537.36 OPR/86.0.2254.74831",
    "systemId": "20915",
    "securityLevel": "L1",
    "manufacturer": "Sharp",
    "model": "SH-51B"
  }
}
```
