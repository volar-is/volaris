 ## What is it?

 volaris-tools is a library used for managing the core logic behind volaris, and any applications that require easy integration with the volaris format.

 ## Security

 volaris-tools is built on top of volaris-crypto - which uses modern, secure and audited<sup>1<sup> AEADs for encryption and decryption.

 You may find the audits for both AES-256-GCM and XChaCha20-Poly1305 on [the NCC Group's website](https://research.nccgroup.com/2020/02/26/public-report-rustcrypto-aes-gcm-and-chacha20poly1305-implementation-review/).

 <sup>1</sup> Deoxys-II-256 does not have an official audit, so use it at your own risk

 ## Who uses Volaris-Tools?

 This library is implemented by [volaris](https://github.com/volar-is/volaris), a secure file
 encryption utility.

 This crate was made to separate the logic away from the end-user application.