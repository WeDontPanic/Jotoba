use std::path::Path;

use comrak::ComrakOptions;
use itertools::Itertools;
use once_cell::sync::OnceCell;

pub static NEWS_RETRIEVE: OnceCell<News> = OnceCell::new();

/// Contains a set of News entries ordered by oldest -> newest
#[derive(Default, Debug)]
pub struct News {
    pub entries: Vec<NewsEntry>,
}

#[derive(Default, Debug)]
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
    pub fn load<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
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

        NEWS_RETRIEVE
            .set(News { entries })
            .expect("failed to set news");

        Ok(())
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
pub fn get() -> &'static News {
    unsafe { NEWS_RETRIEVE.get_unchecked() }
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
