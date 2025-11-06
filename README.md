# Nyeki's CLI Search

Welcome to Nyeki's CLI Search! It's a small tool for searching Google from your terminal.

Searching is quite simple:

```sh
search "Stack Overflow"
```

That's it! Read below to learn how to get it set up and running, it has some API setup steps you
must follow:

## Installation

```sh
cargo install nyekis-search
```

Note that you must have cargo installed for that. Otherwise, check this repository's releases for
downloadable binaries.

## Google API Setup

Before being able to search stuff, you'll have to set up a Google API key and create a programmable
search engine from the Google console.

Head over to [this Google documentation page]() to create an API key. If you don't have any Google
Cloud project, create one from the same popup. Grab the generated API key, you may want to store it
in an `.env` file as `GOOGLE_API_KEY=`.

Once that's done, you'll need to create a programmable search engine. Head over to [this Google
Cloud Console page](https://programmablesearchengine.google.com/controlpanel/create) to create a
new programmable search engine (only set a name, leave the rest options as they are by default).
Once created, you'll be shown a success message and given a script tag for JavaScript, which will
look something like this:

```html
<script async src="https://cse.google.com/cse.js?cx=bc4ce04df5518cb73"></script>
<div class="gcse-search"></div>
```

Note the `?cx={CX}` query parameter in that tag? Copy it and save it in that `.env` file as
`GOOGLE_CX=`.

## Running Nyeki's CLI Search

That's it! Once those two environment variables are set, you can run

```sh
search "Best food from Japan"
```

it'll load results and show you a list of options to select. Enjoy!

## Advanced Usage

There's little advanced usage right now. The only argument you may be interested in is the `--safe`
flag which toggles on safe search. For example:

```sh
search "Best food from Japan" --safe
```
