use crate::webserver::prepare_data;
use config::Config;
use types::jotoba::languages::Language;

/// Checks resources and returns `true` if required features are available
pub fn resources() -> bool {
    let res = resources::get();
    if res.check() {
        return true;
    }

    log::error!(
        "Missing required features: {:?}",
        res.missing_but_required()
    );

    false
}

/// Checks integrity of all resources. Jotoba (should) work perfectly
/// if this function does not fail (ignoring all the bugs and ugly code)
pub fn check() {
    let res = check_all();

    if res {
        println!("Success");
    } else {
        println!("Failed");
    }
}

fn check_all() -> bool {
    println!("Loading data");
    let config = Config::new(None).expect("Config invalid");
    prepare_data(&config);

    println!("Testing resources");
    let res = resources();

    println!("Testing indexes");
    let ind = indexes();

    res && ind
}

fn indexes() -> bool {
    words() && names() && sentences() && regex()
}

fn sentences() -> bool {
    let sentence_retrieve = resources::get().sentences();

    for language in Language::iter_word() {
        let foreign = match indexes::get().sentence().foreign(language) {
            Some(f) => f,
            None => return false,
        };

        for id in foreign.get_vector_store().iter().map(|i| i.document.seq_id) {
            if sentence_retrieve.by_id(id).is_none() {
                println!("Sentence index ({language:?}) don't not match");
                return false;
            }
        }
    }

    let jp_index = indexes::get().sentence().native();
    for id in jp_index
        .get_vector_store()
        .iter()
        .map(|i| i.document.seq_id)
    {
        if sentence_retrieve.by_id(id).is_none() {
            println!("Sentence index (Japanese) don't not match");
            return false;
        }
    }

    true
}

fn names() -> bool {
    let name_retrieve = resources::get().names();

    let transcr_index = indexes::get().name().foreign();
    for i in transcr_index
        .get_vector_store()
        .iter()
        .map(|i| i.document.clone().into_iter())
        .flatten()
    {
        if name_retrieve.by_sequence(i).is_none() {
            println!("Foreign name index does not match resources");
            return false;
        }
    }

    let jp_index = indexes::get().name().native();
    for i in jp_index
        .get_vector_store()
        .iter()
        .map(|i| i.document.clone().into_iter())
        .flatten()
    {
        if name_retrieve.by_sequence(i).is_none() {
            println!("Japanese name index does not match resources");
            return false;
        }
    }

    true
}

fn words() -> bool {
    let word_retrieve = resources::get().words();

    for language in Language::iter_word() {
        let w_index = indexes::get()
            .word()
            .foreign(language)
            .expect(&format!("Missing index {:?}", language));

        for vec in w_index.get_vector_store().iter() {
            for word_doc in vec.document.items.iter() {
                if word_retrieve.by_sequence(word_doc.seq_id).is_none() {
                    println!("Word and Index don't match");
                    return false;
                }
            }
        }
    }

    let jp_index = indexes::get().word().native();
    for vec in jp_index.get_vector_store().iter() {
        if word_retrieve.by_sequence(vec.document).is_none() {
            println!("Word and (Japanese) Index don't match");
            return false;
        }
    }

    true
}

fn regex() -> bool {
    let w_retrieve = resources::get().words();

    let regex_index = indexes::get().word().regex();
    for (_, words) in regex_index.iter() {
        if words.iter().any(|i| w_retrieve.by_sequence(*i).is_none()) {
            println!("Regex index invalid");
            return false;
        }
    }

    true
}
