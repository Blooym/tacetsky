# Tacetsky

Automatic posting of Wuthering Waves news to Bluesky. Customized fork of [skywrite](https://github.com/Blooym/skywrite).

## Setup

### Manual

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed and
   in your `$PATH`.
2. Install the project binary

```
cargo install --git https://github.com/Blooym/tacetsky.git
```

3. Copy `.env.example` to `.env` and fill in the values as necessary.
   Information about configuration options can be found in the
   [configuration](#configuration) section.

4. Run the project from the same directory as `.env`

```
tacetsky start
```

## Configuration

Configuration is handled entirely through environment variables or command-line
flags. The available configuration options for the 'start' command are:

- `DATABASE_URL`: The connection string to use when connecting to the sqlite
  database. Supports some connection parameters.
- `TACETSKY_APP_SERVICE`: The full URL to the service to communicate with. Defaults to
  `https://bsky.social`
- `TACETSKY_APP_IDENTIFIER`: The username or email of the application's account.
- `TACETSKY_APP_PASSWORD`: The app password to use for authentication.
- `TACETSKY_DATA_PATH`: The base directory to store things like configuration files and
  other persistent data.
- `TACETSKY_RERUN_INTERVAL_SECONDS`: The interval of time in seconds between checking for news.
- `TACETSKY_NEWS_BACKDATE_HOURS`:  The number of hours in the past the bot should check for news that hasn't been posted. It is recommended to keep this to at least "1" as otherwise posts may get missed.
- `TACETSKY_NEWS_LOCALE`: The locale to use when fetching news posts. Known valid options are "en", "de", "es", "fr", "kr", "zh-tw" and "jp".
- `TACETSKY_DISABLE_POST_COMMENTS`: Whether Bluesky posts should have comments disabled.
- `TACETSKY_POST_LANGUAGES`: A comma-seperated list of languages in **ISO-639-1** to
  classify posts under. This should corrolate to the language of the posts the
  feed is linking to.
