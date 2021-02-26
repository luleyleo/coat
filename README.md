# Coat
Explorations in reactive UI patterns. Largely inspired by [Druid] and [Crochet].

For background, see the blog post [Towards a unified theory of reactive UI] and [Towards principled reactive UI].

## Screenshots

[![play.rs example](https://raw.githubusercontent.com/Finnerale/coat/screenshots/images/play.png)](./examples/play.rs)

## Architecture

Documentation is TODO.

Mostly similar to [Crochet], with some simplifications for easier prototyping.

## License

Except for the files listed below, all files in this repo are released under the MIT license.

Some code has been cut and pasted from Druid, therefore carries a copyright of "The Druid Authors". See the [Druid repo] for the definitive authors list.

- `src/bloom.rs`
- `src/text/*.rs`

[Druid repo]: https://github.com/linebender/druid
[Druid]: https://github.com/linebender/druid
[Crochet]: https://github.com/raphlinus/crochet
[Towards a unified theory of reactive UI]: https://raphlinus.github.io/ui/druid/2019/11/22/reactive-ui.html
[Towards principled reactive UI]: https://raphlinus.github.io/rust/druid/2020/09/25/principled-reactive-ui.html
