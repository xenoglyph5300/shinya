use serenity::{
    client::bridge::gateway::ShardManager,
    model::id::GuildId,
    prelude::{Mutex, TypeMapKey}
};

use dashmap::DashMap;
use reqwest::Client as Reqwest;
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};

/**
 * 
 */
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

/**
 * 
 */
pub struct ConnectionPool;

impl TypeMapKey for ConnectionPool {
    type Value = PgPool;
}

/**
 * 1.
 */
pub struct CommandNameMap;

impl TypeMapKey for CommandNameMap {
    type Value = Arc<Vec<String>>;
}

/**
 * 2.
 */
pub struct ReqwestClient;

impl TypeMapKey for ReqwestClient {
    type Value = Reqwest;
}

/**
 * 3.
 */
pub struct PubCreds;

impl TypeMapKey for PubCreds {
    type Value = Arc<HashMap<String, String>>;
}

/**
 * 4.
 */
pub struct BotId;

impl TypeMapKey for BotId {
    type Value = Arc<HashMap<String, String>>;
}

/**
 * 5.
 */
pub struct PrefixMap;

impl TypeMapKey for PrefixMap {
    type Value = Arc<DashMap<GuildId, String>>;
}

/**
 * 6.
 */
pub struct EmergencyCommands;

impl TypeMapKey for EmergencyCommands {
    type Value = Arc<Vec<String>>;
}