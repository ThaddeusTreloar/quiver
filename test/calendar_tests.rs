use quiver;
use chrono::{
    DateTime,
    offset
};

#[test]
fn send_item()
{
    let item = quiver::shared::calendar::CalendarItem::{
        title: "SomeItem".to_owned(),
        start: DateTime<offset::Utc>::from_str("2022-09-24T12:00:00Z"),
        end: DateTime<offset::Utc>::from_str("2022-09-24T14:00:00Z"),
        guests: (),
        location: quiver::shared::location::Location{
            common_name: "SomePlace".to_owned(),
            coordinate: quiver::shared::location::Coordinate {
                latitude: 5,
                lontitude: 6,
                altitude: 7,
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
    }

    let res = quiver::client::consumer::push(item);
    debug_assert!(res);
}
