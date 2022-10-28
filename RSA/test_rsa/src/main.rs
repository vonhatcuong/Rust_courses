extern crate openssl;

use openssl::rsa::{Rsa, Padding};

fn main() {
    let passphrase = "demo";

    let public_key_pem = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCuUhHB+xFaa64tSlD9sIBb5cY3
+2DDvz7+Vn32nHAzFF9U/YVehvvScNEGu3h7/DVG4PjWw0T5UfOcbTVEfuBmZisX
GHyUvxlcZOmEhIaL5RMeGAGykPvVeY9j6k39X/XRzXL2JxnCpOA/mBLZ7joAg96T
GiAbgJoNyWfBvoubqQIDAQAB
-----END PUBLIC KEY-----";

    let private_key_pem = "-----BEGIN RSA PRIVATE KEY-----
Proc-Type: 4,ENCRYPTED
DEK-Info: AES-128-CBC,8AE0AD20B4490BA63C59DCEA157F16F1

DSZqtWadtbpsrQqvwssqR60rPoJJgc6DvCZaSurm3YhNAxFz7o4SxqvoIHyocFX9
CWI96hUNt+sPTprnVHO4Rz73queo5ZgdS6vcGbQa7eEhD3Rfcdkt7epRCH8/bAen
CRRDicb5+1nRoPM8D3htf0KKIf8Mv6z26BJbMeTXraEKamU0CkzLNjQvn97TikLL
aRz6zM17udJ5ufPJ0aC81t8wVtmkitttYSkt1ibxQJveTm2rQJPo68528Zo4rl+w
17jAYWZD/lyeBUp8XNkV8LxRHYqWWBciyCABRcC+E6nfnvQmtOS7iscn3U7+9qZC
ifzBEMsVdBX6pIPz7sKIC8FJ0p67nMBskgc6y0HjCceMDnIwmt3MrCpxhvTQ/AFj
Lu0vewVa/xbD2c/KbbZHmEV/sDUtIIDZudvsbCqzOGPPWTaMfBhpfRQbAJmfXe/d
3dFhh+ovUoj+XiCN4XXuMgg19xWPE3iqW5zeHGwNSJd5Ij3aINiQdxLXc95natZ3
rIHWQyp6ugnWC3tcbY0DuMYvknEV6NfUKNUnCvjG+CEhnpqbGE7cMPjxz19wFO97
lW7jp+Y7uwfBcAmSTZ+iaKGvq6GKybsKMJyhy4HYl985RR3nmm4HsjuP/LEQh6kA
Tfl7YMcZ+l9uWz6ftbEwf33s3vd8jdYElPKtb1X6ZQjY9c0/M/eN9PbcfS0jeAgL
47DhCPwbuVzsyVCbAP89/89Ik8473AFNC9J9+C0OFoDn/LthEEQSEo2kG5hVC+21
rWFSgYwE+Cgf9jRXI3JyJcjE8FxMhW2XWVlMblNAGM/EXy16G7oYCVpIvOaZv0eH
-----END RSA PRIVATE KEY-----";

    let data = "Hello, world!";

    // Encrypt with public key
    let rsa = Rsa::public_key_from_pem(public_key_pem.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.public_encrypt(data.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
    println!("Encrypted: {:?}", buf);

    let data = buf;

    // Decrypt with private key
    let rsa = Rsa::private_key_from_pem_passphrase(private_key_pem.as_bytes(), passphrase.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.private_decrypt(&data, &mut buf, Padding::PKCS1).unwrap();
    println!("Decrypted: {}", String::from_utf8(buf).unwrap());
}
