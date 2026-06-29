## Translating KelpieBooks

The original language for this system is English and the base for all translations should be the 
English version `en.ftl`.  Using another language as a starting point may propagate mistranslations 
through the system.

### Fluent

We use [Fluent](https://projectfluent.org/) for our translation system.

Fluent is very powerful at supporting accurate idiomatic translation of genders
and plurals. However, we make little to no use of these facilities at this time.

We have added support for language and variants. If you are translating to a new language e.g. Spanish,
you need to add the `es.ftl` file and include all the translation keys. If
you are just adding a variant say `es_MX.ftl`, then you only need to add the keys you
wish to add unique Mexican translations for.

### Tests

If you add a new language, you should also add a test to `shared_core/src/i18n/mod.rs` for the
new language. This test will check all keys are included. Don't do this for a language variant.

### Fonts

Throughout KelpieBooks we use the [Noto](https://fonts.google.com/noto) fonts.

Noto fonts provide 
> harmonious, aesthetic, and typographically correct global communication, in more than 1,000 languages and over 150 writing systems. 
 
We include webfonts in the product which will be automatically downloaded by the
user as required.

The base NotoSans-Bold and NotoSans-Regular font files supply glyphs for most Latin
Greek and Cyrillic based languages.

If you are supplying a translation for a language with a different writing system, 
e.g. Thai or Arabic, then you need to include the font files for those scipts.

Noto fonts can be downloaded from Google's font site, (https://fonts.google.com/noto).

These can be transformed into web fonts, `.woff2` format, using the `woff2_compress` utility and placed in the
`frontend/assets/fonts` directory. You **must** also add to the `frontend/assets/scss/base/_base.scss` file to include these fonts.