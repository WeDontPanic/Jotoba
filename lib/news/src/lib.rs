use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use comrak::ComrakOptions;

#[cfg(feature = "news_inotify")]
use inotify::{EventMask, Inotify, WatchMask};

use itertools::Itertools;
use once_cell::sync::Lazy;

pub static NEWS_RETRIEVE: Lazy<Mutex<Arc<News>>> =
    Lazy::new(|| Mutex::new(Arc::new(News::default())));

/// Contains a set of News entries ordered by oldest -> newest
#[derive(Default, Debug, Clone)]
pub struct News {
    pub entries: Vec<NewsEntry>,
}

#[derive(Default, Debug, Clone)]
pub struct NewsEntry {
    pub id: u32,
    pub title: String,
    pub long: String,
    pub short: String,
    pub creation_time: u64,
    pub was_trimmed: bool,
}

impl News {
    /// Load news from a folder
    pub fn init<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
        let p = path.as_ref().to_str().unwrap().to_string();

        let update = |p: &str| {
            *NEWS_RETRIEVE.lock().unwrap() = Arc::new(Self::load(p).unwrap());
        };

        update(&p);

        #[cfg(feature = "news_inotify")]
        fs_changed_update(p, update);

        Ok(())
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut entries: Vec<NewsEntry> = Vec::new();

        for (pos, file) in std::fs::read_dir(path)?.enumerate() {
            let file = file?;

            let file_name = file.file_name().to_string_lossy().to_string();
            if !file_name.contains(';') {
                continue;
            }

            let mut fn_split = file_name.split(';');
            let creation_time: u64 = fn_split.next().unwrap().parse()?;
            let title = fn_split.join(";");

            let id = pos as u32;

            let (short, long) = parse_markdown(file.path())?;

            entries.push(NewsEntry {
                id,
                title,
                creation_time,
                was_trimmed: short != long,
                long,
                short,
            });
        }

        entries.sort_by(|a, b| a.creation_time.cmp(&b.creation_time));

        let entry_count = entries.len();
        // Only load 15 latest news
        let entries = entries
            .into_iter()
            .skip(entry_count.saturating_sub(15))
            .collect::<Vec<_>>();

        Ok(News { entries })
    }

    /// Returns an iterator over last `limit` news elements from old -> newest
    pub fn last_entries(&self, limit: usize) -> impl Iterator<Item = &NewsEntry> {
        self.entries
            .iter()
            .skip(self.entries.len() - limit.min(self.entries.len()))
    }

    /// Returns a news entry by its ID
    pub fn by_id(&self, id: u32) -> Option<&NewsEntry> {
        self.entries.iter().find(|i| i.id == id)
    }
}

/// Returns a reference to the loaded news entries
#[inline]
pub fn get() -> Arc<News> {
    NEWS_RETRIEVE.lock().unwrap().clone()
}

fn parse_markdown<P: AsRef<Path>>(file: P) -> Result<(String, String), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(file)?;

    let short_md = shorten_markdown(&contents);

    let mut md_options = ComrakOptions::default();
    md_options.render.unsafe_ = true;
    md_options.extension.autolink = true;
    md_options.extension.tasklist = true;
    md_options.extension.strikethrough = true;

    let short_html = comrak::markdown_to_html(&short_md, &md_options);
    let full_html = comrak::markdown_to_html(&contents, &md_options);

    Ok((short_html, full_html))
}

fn shorten_markdown(full: &str) -> String {
    let line_count = full.split('\n').count().max(1);
    let conten_len = utils::real_string_len(full);

    let mut text_iter = full.split('\n').filter(|i| !i.trim().starts_with('#'));

    let out;
    if conten_len > 100 {
        if line_count > 3 {
            out = text_iter.take(3).join("\n");
        } else {
            out = text_iter.join("\n");
        }
    } else {
        out = text_iter.join("\n");
    }

    out
}

#[cfg(feature = "news_inotify")]
fn fs_changed_update<F: Fn(&str) + Send + 'static>(news_folder: String, update: F) {
    std::thread::spawn(move || {
        let mut inotify = Inotify::init().expect("Failed to initialize inotify");

        inotify
            .add_watch(
                &news_folder,
                WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
            )
            .expect("Failed to add inotify watch");

        let mut buffer = [0u8; 4096];
        loop {
            let events = inotify
                .read_events_blocking(&mut buffer)
                .expect("Failed to read inotify events");

            for event in events {
                if !event.mask.contains(EventMask::ISDIR) {
                    println!("update");
                    update(&news_folder);
                }
            }
        }
    });
}
