use serenity::framework::standard::macros::group;

use crate::commands::{
    ciphers::*, config::*, images::*, other::*, support::*, textchannel_send::*, textmod::*,
};

#[group("Master")]
#[sub_groups(
    General,
    Text,
    TextLast,
    Ciphers,
    TextChannelSend,
    Config,
    Support,
    Images
)]
pub struct Master;

#[group]
#[help_available(false)]
#[commands(ping)]
pub struct General;

#[group("Text Modification")]
#[description = "Commands that modify text. \n
Append 1 in the command to use the last message \n
Example: `mockl` mocks the last message"]
#[commands(mock, inv, upp, low_space, bigspace, h4ck, uwu)]
pub struct Text;

#[group]
#[help_available(false)]
#[commands(mockl, invl, uppl, lowl, spacel, bigspacel)]
pub struct TextLast;

#[group("Ciphers")]
#[description = "Commands that encode/decode messages"]
#[commands(b64encode, b64decode)]
pub struct Ciphers;

#[group("Senders")]
#[description = "Commands that send certain messages to channels"]
#[commands(nice, bruh, quote, vibecheck)]
pub struct TextChannelSend;

#[group("Bot Configuration")]
#[description = "Admin/Moderator commands that configure the bot"]
#[commands(prefix, command, resetprefix)]
pub struct Config;

#[group("Support")]
#[description = "Support commands for the bot"]
#[commands(help, support, info)]
pub struct Support;

#[group("Images")]
#[description = "Commands for fetching/sending images"]
#[commands(hug, pet, slap, cry, cringe, gifsearch)]
pub struct Images;
