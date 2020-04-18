
#![allow(non_snake_case)]
use failure::{
    ensure,
    Fallible,
};
use iota_lib_rs::prelude::iota_client;
use iota_streams::app::{
    message::HasLink
};

use iota_streams::app::transport::{
    Transport,
    tangle::client::*
};

use iota_streams::app_channels::{
    api::tangle::{
        Address,
        Author,
        Subscriber
    },
    message,
};
use iota_streams::core::tbits::Tbits;
use iota_streams::protobuf3::types::Trytes;
use std::str::FromStr;


fn example() -> Fallible<()> {
    let mut client = iota_client::Client::new("https://nodes.devnet.iota.org:443");
    let mut send_opt = SendTrytesOptions::default();
    send_opt.min_weight_magnitude = 10;
    let recv_opt = ();

    let mut author = Author::new("DE9OVLDWFLNTKCYPVMRRKMYOEBLBWHFRQERUHAECFSCHSDZODGRDVXPVJJVGKEZOVVHWENPLWFVZWZUNG", 2, true);
    //println!("Channel address = {}", author.channel_address());

    let public_payload = Trytes(Tbits::from_str("PUBLICPAYLOAD").unwrap());
    let masked_payload = Trytes(Tbits::from_str("MASKEDPAYLOAD").unwrap());

    println!("announce");
    let (announcement_address, announcement_tag) = {
        let msg = &author.announce()?;
        println!("  {}", msg.link.msgid.to_string());
        client.send_message_with_options(&msg, send_opt)?;
        (msg.link.appinst.to_string(), msg.link.msgid.to_string())
    };
    let announcement_link = Address::from_str(&announcement_address, &announcement_tag).unwrap();

    println!("sign packet");
    let signed_packet = {
        let msg = author.sign_packet(&announcement_link, &public_payload, &masked_payload)?;
        println!("  {}", msg.link.msgid.to_string());
        client.send_message_with_options(&msg, send_opt)?;
        (msg.link.appinst.to_string(), msg.link.msgid.to_string())
    };
    let signed_packet_link= Address::from_str(&signed_packet.0, &signed_packet.1).unwrap();
    println!("  at {}", signed_packet_link.rel());

    println!("share keyload for everyone");
    let keyload= {
        let msg = author.share_keyload_for_everyone(&announcement_link)?;
        println!("  {}", msg.link.msgid);
        client.send_message_with_options(&msg, send_opt)?;
        (msg.link.appinst.to_string(), msg.link.msgid.to_string())
    };

    let keyload_link= Address::from_str(&keyload.0, &keyload.1).unwrap();

    println!("tag packet");
    let tagged_packet_link = {
        let msg = author.tag_packet(&keyload_link, &public_payload, &masked_payload)?;
        println!("  {}", msg.link.msgid);
        client.send_message_with_options(&msg, send_opt)?;
        msg.link.clone()
    };

    println!("change key");
    let change_key_link = {
        let msg = author.change_key(&announcement_link)?;
        println!("  {}", msg.link.msgid);
        client.send_message_with_options(&msg, send_opt)?;
        msg.link
    };

    Ok(())
}

fn main() {
    let _result = dbg!(example());
}

