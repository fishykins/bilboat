# Bilboat
<div id="top"></div>

[![Minimum Supported Rust Version]]

<!-- ABOUT THE PROJECT -->
## About The Project

Bilboat is a stenographical library, built to enable the embedding of data within a wav file. Encryption layers are provided, but alternative solutions can easily be injected into the workflow. 
A provided "key" will ensure that data is distributed throughout the wav with no distinguishable pattern, and cannot be decoded without the origional key. While this doesn't serve as explicit encryption, it is a good first layer of obfuscation. 


<!-- GETTING STARTED -->
## Getting Started

Clone me! I will be on crates.io soon...

### Optional features

"Encryption" - enabled by default. this feature ensures that when no explicit encryption is provided, aes-siv is added as a robust layer of protection. 

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT)
  or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Contributions are both welcome and appreciated!

Contributions in any form (issues, pull requests, etc.) to this project must
adhere to Rust's [Code of Conduct].

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

If you are a device vendor and you want your gear to be natively supported, please get in touch. 
If you are not a device vendor but you want to send me a device for testing, also get in touch. 
If you are an AC-10 warthog who wants to contribute air support, absolutely get in touch.

<!-- CONTACT -->
## Contact

Caspar Green - caspar.m.green@gmail.com

Project Link: [https://github.com/fishykins/bilboat](https://github.com/fishykins/bilboat)

<p align="right">(<a href="#top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[Minimum Supported Rust Version]: https://img.shields.io/badge/Rust-1.84.1-blue?color=fc8d62&logo=rust
[Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html