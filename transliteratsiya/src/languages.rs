mod armenian;
mod bulgarian;
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
