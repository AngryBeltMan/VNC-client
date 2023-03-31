// use crate::websocket::read::tungstenite::Message;
// use futures_util::{SinkExt, StreamExt, stream::SplitStream};
// use tokio_tungstenite::*;
// use std::marker::Unpin;
// use std::fmt::Debug;
// use futures_util::stream::Stream;
// type DynError = Box<dyn std::error::Error>;
//
// // pub async fn send_image<S>(sink:&mut S,screen:&Screen) -> Result<(),DynError>
// // where S:SinkExt<Message> + Unpin + Debug, <S as futures_util::Sink<Message>>::Error: Debug
// //  {
// //     let image = screen.capture()?.buffer().to_vec();
// //     sink.send(tungstenite::Message::Binary(image)).await.unwrap();
// //     Ok(())
// // }
// pub async fn read_input<T>(stream:&mut SplitStream<T>,/*enigo:&mut Enigo*/) -> Result<(),DynError>
// where T:StreamExt+Debug, <T as Stream>::Item: Debug
// {
//     if let Some(o) = stream.next().await {
//         let o = format!("{:?}",o);
//         let o =o.replace(")", "").replace("(", "").replace("Ok", "").replace("Text","");
//         // if o.contains("MOUSE") {
//         //     let o = o.replace("MOUSE", "");
//         //     let o = o.split(",").collect::<Vec<&str>>();
//         //     let x = o[0].parse::<i32>()?;
//         //     let y = o[1].parse::<i32>()?;
//         //     enigo.mouse_move_to(x, y)
//         // } else if o.contains("SCROLLY") {
//         //     enigo.mouse_scroll_y(o.replace("SCROLLY", "").parse().unwrap());
//         // } else if o.contains("SCROLLX") {
//         //     enigo.mouse_scroll_x(o.replace("SCROLLX", "").parse().unwrap());
//         // } else if o.contains("LEFT") {
//         //     enigo.mouse_click(MouseButton::Left)
//         // } else if o.contains("RIGHT") {
//         //     enigo.mouse_click(MouseButton::Right)
//         // } else if o.len() == 1 {
//         //     enigo.key_sequence(&o);
//         // }
//     }
//     Ok(())
// }
// pub async fn test<S>(text:String,sink:&mut S) -> Result<(),DynError>
// where S:SinkExt<Message> + Unpin + Debug, <S as futures_util::Sink<Message>>::Error: Debug
//  {
//     sink.send(tungstenite::Message::Text(text)).await.unwrap();
//     Ok(())
// }
