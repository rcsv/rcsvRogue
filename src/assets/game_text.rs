//! game_text.rs
//! src/assets/game_text.rs
//!
//! $Gcid$
//!
//! ちょっと目的がよくわからないモジュール。TOML フォーマット
//! で成型したメッセージを発行する .properties 代行？

use std::io::Result; // Resultset はよく使う。

use ggez::graphics::{Font, Text};
use ggez::{Context, GameResult};
use toml;

use assets::Assets;
use etc;

#[derive(Clone, Debug, Deserialize)]
pub struct Source {
    title_logo:         String,
    title_description:  String,
    title_headline:     String,
    title_tips:         Vec<String>,
    game_over_title:    String,
    game_over_score:    String,
    game_over,tips:     Vec<String>,
}
impl Source {
    fn new(assets: &Assets) -> Result<Self> {
        let mut text_path = assets
            .show_map()
            .get("game_text.toml") // どこかに game_text.toml がないかな？
            .expect("HashMap.get() のエラー")
            .strip_prefix("/") // directory separator 削除
            .expect("strip_prefix() のエラー")
            .to_str()
            .expect("PathBuf.to_str() のエラー");
        let mut text_path = etc::easy_path_set(text_path);
        let tmp_vec = etc::File::read_to_vec(&text_path)?; // この末尾の ? はなんだろね。
        let src_text: Source = toml::de::from_slice(&tmp_vec)
            .expect("toml deserialization error");
        Ok(src_text)
    }
}

// public interface IGameText みたいなものなのかな
#[derive(Clone, Debug)]
pub struct GameText {
    pub title_logo:         Text,
    pub title_description:  Text,
    pub title_headline:     Text,
    pub title_tips:         Vec<Text>,
    pub game_over_title:    Text,
    pub game_over_score:    Text,
    pub game_over_score_num:Text,
    pub game_over_tips:     Vec<Text>,
}
// private class GameText implements IGameText 
// のようなものと解釈している
impl GameText {

}