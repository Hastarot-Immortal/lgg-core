pub mod sound;
pub mod word;
pub mod dictionary;
pub mod dictionary_iter;
// pub mod language;
pub mod rule;

pub use sound::{ Sound, VoiceLevel };
pub use word::{ Word, PartOfSpeech };
pub use dictionary::Dictionary;
pub use dictionary_iter::{ Words, WordsMut, DictionaryMapIter, DictionaryMapIterMut };
// pub use language::{ Language, LanguageBuilder, RandomLanguageBuilder };
pub use rule::Rule;

pub use cc_traits::{
	// Immutable traits
	Collection, 
    Keyed,
    KeyedRef,
    SimpleKeyedRef,
    Len, 
    CollectionRef,
    SimpleCollectionRef,
    Get,
    GetKeyValue,
    Map, 

    // Mutable traits
    CollectionMut,
    SimpleCollectionMut,
    GetMut,
    MapInsert,
    Remove,
    Clear,
	MapMut,

    // Iterators
    Iter,
	IterMut,
	MapIter,
	MapIterMut,
};