mod armenian;
mod bulgarian;
mod greek;
mod latin1;
mod russian;
mod serbian;
mod ukranian;

/// See
/// [https://en.wikipedia.org/wiki/Armenian_alphabet](https://en.wikipedia.org/wiki/Armenian_alphabet)
/// for details.
///
/// Based on the work by Artur Barseghyan.
pub use armenian::Armenian;

/// See
/// [http://en.wikipedia.org/wiki/Romanization_of_Bulgarian](http://en.wikipedia.org/wiki/Romanization_of_Bulgarian)
/// for details.
///
/// Based on the work of Petar Chakarov.
pub use bulgarian::Bulgarian;

/// See
/// [http://en.wikipedia.org/wiki/Greek_alphabet](http://en.wikipedia.org/wiki/Greek_alphabet)
/// and
/// [https://en.wikipedia.org/wiki/Romanization_of_Greek#Modern_Greek](https://en.wikipedia.org/wiki/Romanization_of_Greek#Modern_Greek)
/// for details.
///
/// Based on the work of Artur Barseghyan.
pub use greek::Greek;

/// Though not exactly a language, it's a set of commonly found unicode
/// characters. See
/// [http://en.wikipedia.org/wiki/Latin-1_Supplement_%28Unicode_block%29](http://en.wikipedia.org/wiki/Latin-1_Supplement_%28Unicode_block%29)
/// for details.
///
/// Based on the work of Marco Pattaro.
pub use latin1::Latin1;

/// See
/// [http://en.wikipedia.org/wiki/Russian_alphabet](http://en.wikipedia.org/wiki/Russian_alphabet)
/// for details.
///
/// Based on the work of Artur Barseghyan.
pub use russian::Russian;

/// See
/// [https://en.wikipedia.org/wiki/Romanization_of_Serbian](https://en.wikipedia.org/wiki/Romanization_of_Serbian)
/// for details.
///
/// Based on the work of Saša Kelečević.
pub use serbian::Serbian;

/// See
/// [http://en.wikipedia.org/wiki/Ukrainian_alphabet](http://en.wikipedia.org/wiki/Ukrainian_alphabet)
/// for details.
///
/// Based on work by Timofey Pchelintsev.
pub use ukranian::Ukranian;
