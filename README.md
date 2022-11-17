# Quiver

Quiver is an ambitious project aiming to create a cross-platform, standardised interface for common service types.
The intent is to seperate the duties of front and back end services to allow for back-end service aggregation and composition by user applications.
This description may be vague however, I will use two examples to illustrate the vision I have for these interfaces.

## Example 1: Maps Services.

### The current state: 
This example is intended to showcase the extensibility Quiver would provide to front-end user applications and smaller freeware services.
'XYZ Maps' is a consumer maps and navigation application. XYZ mantains and updates all maps itself. 
Users are able to add their business locations or provide picutres of locations but this is is all maintained by XYZ.
To add any information other than the set of user-sourced data-points provided by XYZ one must contact the company, submit a request, and hope that XYZ have the time and budget available to fulfill their request.
'Hiker.io' is a web application that provides user-sourced and moderated maps for hiking trails. 
It provides no funcitonality for navigation and integrates with no other application. To save information about a trail a user must screenshot the data and navigate themselves to the start of the trail, sometimes facing obscure directions.
Whilst a fantastic source of information, this web-app lacks some functionality; functionality that would be expensive and time consuming to implement considering it is a free-to-use platform with no monetisation.

### Where Quiver comes in:
Rather than XYZ having to create a monolithic application for maps, we can split duties between several smaller applications.

#### Service interface:
Quiver provides an interface where a data producer can indicate to the QuiverCore services that it implements an interface and provides data via it.
In the case of a hypothetical maps interface pieces of data may exists in the following format:

MapsDataPoint {
  MapPointType, # Enum
  LocationSpline, # Vec<struct Location> of len [1..]
  ID, # String
  MetaData, # JSon (service specific including information about how it should be presented)
}

The method to access this information from the service provider is an arbitrary IPC method, but could be a Unix local socket, TCP socket, Bind address, Pipe, DBus.
This would depend on the platform and interface requirements but would be provided by a library and abstracted from the developer.

Data consumers would then be able to access data using a constrained one-to-many query via the QuiverCore service.

MapsQuery {
  MapPointType, # Option<Vec<Enum>>
  LocationBounds, # struct LocationBound
}

Again, the IPC mechanism is arbitrary and would be abstracted from the devleoper by the library.

This would allow both 'XYZ maps' and 'Hiker.io' to create maps services that implement the interface required by QuiverCore to provide their data to any Maps Consumer.
'XYZ maps' would then create a front-end user application that implements the data consumer interfaces for QuiverCore.
Then when requesting data to be rendered by the application,  it would request directly to QuiverCore, which would in turn request from all reqistered data producers, in this case 'XYZ maps' and 'Hiker.io', before returning any data as a query to the 'XYZ maps' front-end application.
'XYZ maps' front end would then collate, and render the data in a unified that would be seamless to the user experience.
It allows 'XYZ maps' to improve their user experience without the burden of maintaining extra datasets.

## Example 2: Concert ticket provider.
This example is intended to showcase the development flexibility and integration Quiver would allow developers.
'ConcertTIX' is a company that provides tickets for local concerts.
'ConcertTIX' does not provide a front-end application, only a service that implements serveral Quiver interfaces to achieve it's funcitonality.
The interfaces will be presented in short-hand for brevity:

  Quiver::Producer::Shopping -> {
    provides item listings for products or services,
    any application that consumes this interface will have access to this data,
    each ticket is a product which is then indexed and rendered by a third party front end.
  }
  Quiver::Producer::Payments -> {
    creates payment requests,
    any service that a user has set up to accept payment requests will be able to process a payment created via this interface
  }
  Quiver::Producer::Invoicing -> {
    issues payment invoices,
    services that may consumer this data would be bookeeping/accounting software or personal finance services
  }
  Quiver::Producer::Wallet -> {
    issues authentication or authorisation data,
    coulde be a barcode or QR with a small amount of information,
    could even be something like a publickey in higher security situations (employee workplace access)
  }
  Quiver::Producer::Calendar -> {
    provides events,
    other than the obvious name/data/time this would also refer to other quiver interfaces eg:
      Guest: Vec<struct SocialProfileLink>>
      Location: struct MapsDataPoint
      Attachment: Vec<struct CloudStorageFileReference>
      Action: MeetingLink, PhoneCall
  }
  Quiver::Producer::Social -> {
    forwards references to other interfaces to friends
    services could reserve a ticket for a specified amount of time and send a reference to a friend for them to purchase
    or if purchasing a ticket to friends, transfer their ticket to them as if they had bought it.
  }
  Quiver::Consumer::Social -> {
    uses the user's data for autofilling of form data for invoices etc.
  }

After implementing such interfaces 'ConcertTIX' would then have robust interoperability with all relevant services on the user's device.
It's hard to understate how powerful this integration would be.
