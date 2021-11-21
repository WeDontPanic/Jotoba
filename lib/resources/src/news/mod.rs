use std::path::Path;

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
            .ok()
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

    let short_html = markdown::to_html(shorten_markdown(&contents)).replace("\n", "<br>");
    let full_html = markdown::to_html(&contents).replace("\n", "<br>");

    Ok((short_html, full_html))
}

fn shorten_markdown(full: &str) -> &str {
    let line_count = full.split('\n').count().max(1);
    let conten_len = utils::real_string_len(full);

    let mut end = full.len().min(50);

    if conten_len > 100 {
        if line_count > 3 {
            end = full.split('\n').take(3).map(|i| i.len()).sum();
        }
    }

    &full[..end]
}
