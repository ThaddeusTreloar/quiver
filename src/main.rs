mod core;
mod security;
mod handler;
mod shared;
use env_logger;
mod client;

fn main() {
    env_logger::init();
    let config: core::config::CoreConfig = core::init::init();
    match core::main::main(config) {
        Ok(_val) => return,
        Err(_err) => unimplemented!(),
    }
}




#[test]
fn send_item()
{
    use chrono::{
        DateTime,
        offset,
    };
    let item = shared::calendar::CalendarItem{
        title: "SomeItem".to_owned(),
        start: "2022-09-24T12:00:00Z".parse::<DateTime<offset::Utc>>().unwrap(),
        end: "2022-09-24T14:00:00Z".parse::<DateTime<offset::Utc>>().unwrap(),
        guests: (),
        location: shared::location::Location{
            common_name: "SomePlace".to_owned(),
            coordinate: shared::location::Coordinate {
                latitude: 5.0,
                lontitude: 6.0,
                altitude: 7.0,
            },
            address: "SomeAddress".to_owned(),
        },
        description: "An Event".to_owned(),
        // Attachments will link to either a sharing link or local file.
        // Sharing links will be visible to all with permissions to read
        // it. Maybe add something that automatically offers to add permissions to guests.
        // Local file will only be available to owner. Maybe add automatic generation
        // of a sharing link eg. local_file->upload_to_cloud->generate_link
        attachments: (),
        // Action will link to some action from another interface eg. Call, meeting link etc..
        action: (),
        // Guests will be a link to INS or contact or something
        owner: ()
    };

    let res = client::consumer::calendar::push(item);
    assert_eq!(res, Ok(()));
}
