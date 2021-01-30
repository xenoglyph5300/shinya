## Shinya
A Discord bot repository based on Rust. Unusable state and still heavily under construction.

#### Library Used
- [serenity](https://github.com/serenity-rs/serenity) : Provide interface framework for Discord API in Rust language.
- [sqlx](https://github.com/ant32/sqlx) : Command utility for managing database ORM query.
- [Tokio](https://github.com/tokio-rs/tokio) : Asynchronous runtime written in Rust language.
- [lavalink-rs](https://gitlab.com/nitsuga5124/lavalink-rs/) : Asynchronous rust crate for Lavalink audio node binder. (To be considered)
- [Lavalink](https://github.com/Frederikam/Lavalink) : Audio node based on Lavaplayer. (To be considered)

#### Prerequisite
- [Rust Toolchain](https://www.rust-lang.org/tools/install)
- [Postgres12+](https://www.postgresql.org/download/)
- [SQLx CLI](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli#install)
- [Java JDK13+](https://www.oracle.com/java/technologies/javase-jdk13-downloads.html) (for Lavalink usage)

#### How to Use
**1.** Configure postgres url `DATABASE_URL` and `DISCORD_TOKEN` that can be found at Discord's Developer Portal in `.env`

**2.** To run migration script in migration folder

```
sqlx database create
```
&emsp;or

```
sqlx migrate run
```
&emsp;These will create `sqlx-data.json`

**3.** Run

```
cargo run
```
**4.** To be updated

#### Special Thanks to
_Depression and cats._
