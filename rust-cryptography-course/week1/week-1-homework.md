# Week 1 Homework

Author: dcbuild3r

- [x] Ch 1: 
  - [x] Q10. Describe a concrete example where improving the security of a system against one type of attack can increase the likelihood of other attacks.
  - hiring a customer support department to retrieve login information instead of having an email-based recovery mechanism might decrease security because they might get socially engineered to give private information to malicious actors instead of keeping the system more secure.
- [x]  Ch 2:
  - [x]   Q3. Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. How many keys need to be exchanged in total.
  - 435
  - [x]   Q4. Suppose Bob receives a message signed using a digital signature scheme with Alice’s secret signing key. Does it prove that Alice saw the message and chose to sign.
  - No, someone might have spoofed $K_{a}$
  - [x]   Q6. Suppose a chosen-ciphertext attacker cannot recover the secret decryption key for an encryption scheme. Does this mean the encryption scheme is secure?
  - No, the attacker might still recover information about the message without being able to find the key.
  - [x]   Q7. Consider a symmetric-key cryptosystem in which cryptographic keys are randomly selected from the set of all n-bit strings. Approximately what should n be in order to provide 128 bits of security against a birthday attack.
  - 256, $2^{n/2}$ -> $2^{256/2}=2^{128}$
- [x] General:
  - [x]   Find two libraries for each of RSA, TLS/SSL, and AEAD. Evaluate the maturity each library, and skim the code. What about the library structure makes sense? How is their documentation? These links may help:
  - [x]   [https://cryptography.rs/](https://cryptography.rs/)
  - [x]   [https://lib.rs/](https://lib.rs/) (librs is equivalent to [crates.io](http://crates.io/), with a different interface)
  - The RustCrypto, Ring, RusTLS, and Orion.
    - TLS - RusTLS, openSSL
    - RSA - rsa, Ring
    - AEAD - RustCrypto, AEADs/aes-gcm
  - [x]   Benchmark the speed of an algorithm in the two different implementations with [Criterion](https://lib.rs/crates/criterion) or [iai](https://lib.rs/crates/iai). You may use this [code snippet](https://github.com/thor314/tmpl/blob/main/base/benches/bench.rs#L19) for reference. (skipped - straightforward)
  - [x]   You want to understand a paper on a new polynomial commitment scheme, but you’ve been trying for more than an hour, and the math is over your head. What do you do?
  - explainpaper.com, ask friends on Twitter, DM Thor, ask coworkers, try again, repeat
  - [x]   Implement the [Vignère cipher](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher) in 100 lines or less.
  - [x]   What is a side channel attack? Is your cipher implementation constant time?
  - A side channel attack is any attack that can be performed using extra knowledge gathered about the cryptographic protocol or its implementation.
  - [x]   Extra: Read [New Directions in Cryptography](https://ieeexplore.ieee.org/document/1055638).
  - [x]   Extra: Consider ways to contribute what you learned this week to the [Uncloak](https://uncloak.org/) knowledge graph
