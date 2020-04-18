
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
    let address = "";
    send_opt.min_weight_magnitude = 10;
    let recv_opt = ();

    let mut subscriber = Subscriber::new("SDMFMMWDMEILAXZWILTNVMTMSFK9DWEXTYQH9HQICXATVVQECIDDYLVDKKI9WRNIVNACUWNQQLBPN9BPU", false);

    println!("announce");

    let announcement_tag = "DGN9ZJALPKY9AKCNJIQAXYWT9ZS";
    let announcement_link = Address::from_str(&address, &announcement_tag).unwrap();

    {
        let msg = client.recv_message_with_options(&announcement_link, recv_opt)?;
        let preparsed = msg.parse_header()?;
        ensure!(preparsed.check_content_type(message::announce::TYPE));

        subscriber.unwrap_announcement(preparsed.clone())?;
    }

    println!("sign packet");
    let sign_packet_tag = "9MUUJREJQQZLRYFTBGHOS9FLGBV";
    let signed_packet_link= Address::from_str(&address, &sign_packet_tag).unwrap();
    println!("  at {}", signed_packet_link.rel());

    {
        let msg = client.recv_message_with_options(&signed_packet_link, recv_opt)?;
        let preparsed = msg.parse_header()?;
        ensure!(preparsed.check_content_type(message::signed_packet::TYPE));
        let (unwrapped_public, unwrapped_masked) = subscriber.unwrap_signed_packet(preparsed)?;
    }

    println!("subscribe");
    let subscribe_link = {
        let msg = subscriber.subscribe(&announcement_link)?;
        println!("  {}", msg);
        client.send_message(&msg)?;
        Address::from_str(&msg.link.appinst.to_string(), &msg.link.msgid.to_string()).unwrap()
    };

    {
        let msg = client.recv_message(&subscribe_link)?;
        let preparsed = msg.parse_header()?;
        ensure!(preparsed.check_content_type(message::subscribe::TYPE));
    }

    println!("share keyload for everyone");
    let keyload_tag = "EOKIESQZKMNVSZAFMOODAIPYEVG";
    let keyload_link= Address::from_str(&address, &keyload_tag).unwrap();

    {
        let msg = client.recv_message_with_options(&keyload_link, recv_opt)?;
        let preparsed = msg.parse_header()?;
        ensure!(preparsed.check_content_type(message::keyload::TYPE));
        subscriber.unwrap_keyload(preparsed.clone());
    }

    println!("tag packet");

    let tagged_packet_tag = "CS9NBZPAQI9YOZWMGFGHRDBBKZM";
    let tagged_packet_link = Address::from_str(&address, &tagged_packet_tag).unwrap();
    {
        let msg = client.recv_message_with_options(&tagged_packet_link, recv_opt)?;
        let preparsed = msg.parse_header()?;
        ensure!(preparsed.check_content_type(message::tagged_packet::TYPE));
        let result = subscriber.unwrap_tagged_packet(preparsed.clone());
    }

    {
        let tagged_packet = client.recv_message_with_options(&tagged_packet_link, recv_opt)?;
        let preparsed = tagged_packet.parse_header()?;
        ensure!(preparsed.check_content_type(message::keyload::TYPE));
    }

    /*println!("change key");

    let change_key_tag = "";
    let
    {
        let msg = client.recv_message_with_options(&change_key_link, recv_opt)?;
        let preparsed = msg.parse_header()?;
        ensure!(preparsed.check_content_type(message::change_key::TYPE));
    }*/

    Ok(())
}

fn main() {
    let _result = dbg!(example());
}

