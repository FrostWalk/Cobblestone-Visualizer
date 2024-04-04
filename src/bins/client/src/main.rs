use exclusion_zone::content::bank::BankSettings;
use exclusion_zone::content::bin::BinSettings;
use exclusion_zone::content::coin::CoinSettings;
use exclusion_zone::content::fire::FireSettings;
use exclusion_zone::content::fish::FishSettings;
use exclusion_zone::content::garbage::GarbageSettings;
use exclusion_zone::content::market::MarketSettings;
use exclusion_zone::content::rock::RockSettings;
use exclusion_zone::content::tree::TreeSettings;
use exclusion_zone::content::wood_crate::CrateSettings;
use exclusion_zone::generator::{get_default_spawn_order, NoiseSettings, Thresholds, WorldGenerator};
use exclusion_zone::tile_type::lava::LavaSettings;
use message_io::network::{NetEvent, Transport};
use message_io::node;
use message_io::node::{NodeEvent, NodeHandler, NodeListener};
use walle_visualizer::compressor::Compressor;
use walle_visualizer::messages::{Command, Response};

const ADDRESS: &str = "0.0.0.0:3439";

enum Signal {
    StartWG,
    Stop,
}

async fn run() {
    let world_size = 100;

    let world_generator = WorldGenerator::new(
        world_size,
        get_default_spawn_order(),
        NoiseSettings::default(),
        Thresholds::default(),
        LavaSettings::default(world_size),
        BankSettings::default(world_size),
        BinSettings::default(world_size),
        CrateSettings::default(world_size),
        GarbageSettings::default(world_size),
        FireSettings::default(world_size),
        TreeSettings::default(world_size),
        CoinSettings::default(world_size),
        MarketSettings::default(world_size),
        FishSettings::default(world_size),
        RockSettings::default(world_size),
    );

    let m = Command::CreateWorld(Some(Box::new(
        world_generator
    )));
    
    let (handler, listener): (NodeHandler<Signal>, NodeListener<Signal>) = node::split();
    let h_copy = handler.clone();

    let (server_id, _) = handler.network().connect(Transport::Tcp, ADDRESS).unwrap();

    let _ = listener.for_each_async(move |event| match event {
        NodeEvent::Network(e) => {
            match e {
                NetEvent::Connected(..) =>{
                    println!("connected");
                    handler.signals().send(Signal::StartWG);

                }
                NetEvent::Message(_, payload) => {
                    let response: Response = bincode::deserialize(payload).unwrap();
                    if let Response::Status(e) = response {
                        println!("Received: {:?}", e);
                    }
                }
                _ => ()
            }
        }
        NodeEvent::Signal(e) => {
            match e {
                Signal::StartWG => {
                    h_copy.network().send_compressed(server_id, &m).expect("TODO: panic message");
                }
                Signal::Stop => {
                    h_copy.stop();
                }
            }
        }
    });

}

#[tokio::main]
async fn main() {
    run().await;
}