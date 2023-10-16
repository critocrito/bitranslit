<a name="readme-top"></a>


[![Crates.io][crates-io-shield]][crates-io-url]
[![Docs.rs][docs-rs-shield]][docs-rs-url]
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![CI][ci-shield]][ci-url]
[![LGPL 2.1 License][license-shield]][license-url]


<br />
<div align="center">
  <h3 align="center">Bitranslit</h3>

  <p align="center">
    Bi-directional transliterator for Rust.
    <br />
    <a href="https://github.com/critocrito/bitranslit/issues">Report Bug</a>
    ·
    <a href="https://github.com/critocrito/bitranslit/issues">Request Feature</a>
  </p>
</div>


<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>


## About The Project

Bi-directional transliterator for Rust. Transliterates strings according to the rules specified in the language packs (source script <-> target script) and transliteration standards.

This crate uses the transliteration rules from the [Python `transliterate` package](https://github.com/barseghyanartur/transliterate) by Artur Barseghyan. 

It comes with language packs for the following languages:

- Armenian
- Bulgarian (beta)
- Greek
- Latin1
- Makedonian (alpha)
- Russian
- Serbian (alpha)
- Ukrainian (beta)

This code was developed to support investigations that took place at [Der SPIEGEL](https://www.spiegel.de) and [Paper Trail Media](https://www.papertrailmedia.de).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Getting Started

### Installation

```sh
cargo add bitranslit
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Usage


``` rust
use bitranslit::Bulgarian;

let t = Bulgarian::new();

let _ = t.from_latin("Lorem ipsum dolor sit amet");
let _ = t.to_latin("Лорем ипсум долор сит амет");
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## License

Distributed under the LGPL-2.1-or-later License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Contact

Christo Buschek - [@christo_buschek](https://twitter.com/christo_buschek) - christo.buschek@proton.me

Project Link: [https://github.com/critocrito/bitranslit](https://github.com/critocrito/bitranslit)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



[contributors-shield]: https://img.shields.io/github/contributors/critocrito/bitranslit.svg?style=for-the-badge
[contributors-url]: https://github.com/critocrito/bitranslit/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/critocrito/bitranslit.svg?style=for-the-badge
[forks-url]: https://github.com/critocrito/bitranslit/network/members
[stars-shield]: https://img.shields.io/github/stars/critocrito/bitranslit.svg?style=for-the-badge
[stars-url]: https://github.com/critocrito/bitranslit/stargazers
[issues-shield]: https://img.shields.io/github/issues/critocrito/bitranslit.svg?style=for-the-badge
[issues-url]: https://github.com/critocrito/bitranslit/issues
[license-shield]: https://img.shields.io/github/license/critocrito/bitranslit.svg?style=for-the-badge
[license-url]: https://github.com/critocrito/bitranslit/blob/master/LICENSE.txt
[crates-io-shield]: https://img.shields.io/crates/v/bitranslit.svg?style=for-the-badge
[crates-io-url]: https://crates.io/crates/bitranslit
[docs-rs-shield]: https://img.shields.io/docsrs/bitranslit?style=for-the-badge
[docs-rs-url]:https://docs.rs/bitranslit
[ci-shield]: https://img.shields.io/github/actions/workflow/status/critocrito/bitranslit/CI?style=for-the-badge
[ci-url]: https://github.com/critocrito/bitranslit/actions
