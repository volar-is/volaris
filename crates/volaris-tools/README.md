 ## What is it?

 volaris-tools is a library used for managing the corecrypto logic behind volaris, and any applications that require easy integration with the volaris format.

 ## Security

 volaris-tools is built on top of volaris-crypto - which uses modern, secure and audited<sup>1<sup> AEADs for encryption and decryption.

 You may find the audits for both AES-256-GCM and XChaCha20-Poly1305 on [the NCC Group's website](https://research.nccgroup.com/2020/02/26/public-report-rustcrypto-aes-gcm-and-chacha20poly1305-implementation-review/).

 ## Who uses Volaris-Tools?

 This library is implemented by [volaris](https://github.com/volar-is/volaris), a secure command-line file
 encryption utility.

 This crate was made to separate the logic away from the end-user application.

 It also allows for more things to be built on top of the corecrypto functionality, such as a GUI application.


 You can read more about volaris, volaris-crypto, volaris-tools and the technical details [in the project's main documentation](https://github.com/volar-is/volaris/)!
