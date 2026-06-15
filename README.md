# lgg_core

`lgg_core` is a structural linguistic engine and phonetic framework designed for building programmatic dictionary systems, simulating conlangs (constructed languages), and applying deterministic sound change rules.

## Core Architecture

The framework is structured into several interconnected modules:
- `sound`: Defines the fundamental phoneme units (`Sound`) and their acoustic voicing levels.
- `word`: Handles vector sequences of sounds with attached grammatical classifications (`Word`, `PartOfSpeech`).
- `dictionary`: Provides efficient unique-key lookup collections (`Dictionary`) to store synthesized words.
- `language`: Wraps vocabulary storage and provides builders/extenders to manage language state.
- `rule`: Implements stateless and stateful rule-based transformers (`Rule`) to mutate words in-place.

## Features

- `alphabet`: Offers an optional ordered phonetic framework pool to index, filter, and validate sounds.
