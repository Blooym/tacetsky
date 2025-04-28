use crate::database::Database;
use anyhow::Result;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use reqwest::Url;
use serde::Deserialize;
use tracing::debug;

pub struct WuwaNewsFetcher<'a> {
    filter_date: DateTime<Utc>,
    database: &'a Database,
    backdate_duration: Duration,
    news_url: Url,
    locale: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WuwaRoot {
    pub article_type: Vec<WuwaArticleType>,
    pub pc_top_picture: WuwaTopPicture,
    pub article: Vec<WuwaArticle>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WuwaArticleType {
    pub content_id: u32,
    pub content_label: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WuwaTopPicture {
    pub cover_image: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WuwaArticle {
    pub article_id: u32,
    pub article_title: String,
    pub article_type: u32,
    pub create_time: String,
}

#[derive(Debug)]
pub struct WuwaNewsPost {
    pub url: Url,
    pub title: String,
    pub publish_time: DateTime<Utc>,
    pub cover: Url,
    pub description: String,
    pub content_tag: Option<String>,
}

impl<'a> WuwaNewsFetcher<'a> {
    fn make_news_url(locale: &str, timestamp: i64) -> Url {
        Url::parse(&format!(
            "https://hw-media-cdn-mingchao.kurogame.com/akiwebsite/website2.0/json/G152/{}/MainMenu.json?t={}",
            locale, timestamp
        ))
        .expect("static parsed url should always be valid")
    }

    pub fn new(locale: String, database: &'a Database, feed_backdate: Duration) -> Self {
        let now = Utc::now();
        let news_url = Self::make_news_url(&locale, now.timestamp_millis());
        let filter_date = now - feed_backdate;
        debug!(
            "Initializing news fetcher for {news_url} with starting filter date of {filter_date}"
        );

        Self {
            database,
            news_url,
            filter_date,
            locale,
            backdate_duration: feed_backdate,
        }
    }

    pub fn get_news_url(&self) -> &Url {
        &self.news_url
    }

    pub async fn fetch_unposted(&mut self) -> Result<Vec<WuwaNewsPost>> {
        let mut content = reqwest::get(self.news_url.as_str())
            .await?
            .json::<WuwaRoot>()
            .await?;

        content.article = content
            .article
            .into_iter()
            .filter(|f| f.article_type != 0)
            .collect();
        content.article.dedup_by_key(|f| f.article_id);
        content.article.sort_by_key(|f| f.article_id);
        content.article.reverse();

        let mut posts = vec![];
        for item in content.article {
            let create_time = NaiveDateTime::parse_from_str(&item.create_time, "%Y-%m-%d %H:%M:%S")
                .expect("datetime format should be pre-validated")
                .and_utc();

            // Only count posts that are after the filter date.
            if create_time <= self.filter_date {
                continue;
            }

            let link = Url::parse(&format!(
                "https://wutheringwaves.kurogames.com/{}/main/news/detail/{}",
                self.locale, item.article_id
            ))?;
            if self.database.has_posted_url(link.as_str()).await? {
                continue;
            }

            posts.push(WuwaNewsPost {
                cover: content.pc_top_picture.cover_image.clone(),
                description: "".to_string(),
                publish_time: create_time,
                title: item.article_title,
                content_tag: content
                    .article_type
                    .iter()
                    .find(|f| f.content_id == item.article_type)
                    .map(|a| a.content_label.clone()),
                url: link,
            });
        }
        self.filter_date = Utc::now() - self.backdate_duration;
        Ok(posts)
    }
}
