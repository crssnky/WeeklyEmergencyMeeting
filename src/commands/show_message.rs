use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::{channel::ReactionType, prelude::*};
use serenity::prelude::*;

use std::convert::TryFrom;

use crate::store::message_store::MessageStore;

extern crate ini;
use ini::Ini;

#[command]
async fn show_message(ctx: &Context, msg: &Message) -> CommandResult {
  let message_lock = {
    let data_read = ctx.data.read().await;
    data_read
      .get::<MessageStore>()
      .expect("Expected MessageStore in TypeMap.")
      .clone()
  };
  let mes = message_lock.read().await;
  if mes.len() == 0 {
    msg.reply(ctx, "<!not set message!>").await?;
  } else {
    let new_msg = msg.reply(ctx, mes).await?;
    let conf = Ini::load_from_file("config.ini").unwrap();
    let section = conf.section(Some("Discord")).unwrap();
    let reacts = section
      .get("reacts")
      .unwrap()
      .split(",")
      .collect::<Vec<&str>>();
    for react in reacts {
      let reaction = ReactionType::try_from(react);
      match reaction {
        Ok(r) => {
          println!("react is {:?}", r);
          match new_msg.react(ctx, r).await {
            Ok(res) => {
              println!("react: {:?}", res);
            }
            Err(err) => {
              println!("react message: {:?}", err);
            }
          }
        }
        Err(err) => {
          println!("try_from react: {:?}", err);
        }
      }
    }
  }
  Ok(())
}
