use crate::store::message_store::MessageStore;
use serenity::{
  http::Http,
  model::id::ChannelId,
  prelude::{RwLock, TypeMap},
};
use std::sync::Arc;
use std::vec::Vec;

pub async fn say_announce(
  c: &ChannelId,
  reacts: &Vec<&str>,
  data: &Arc<RwLock<TypeMap>>,
  http: &Arc<Http>,
) {
  let mes_lock = {
    let data_read = data.read().await;
    data_read
      .get::<MessageStore>()
      .expect("Expected MessageStore in TypeMap.")
      .clone()
  };
  let mes = mes_lock.read().await;
  println!("Start announce: {:?}", mes);
  match c.say(&http, mes).await {
    Ok(mes) => {
      println!("send message: {:?}", mes);
      // React own message
      for _react in reacts {
        let _ = mes.react(&http, '⏱').await;
      }
    }
    Err(err) => {
      println!("send message: {:?}", err);
    }
  }
}
