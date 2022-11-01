// Internal
use crate::{
    connection::{
        authrorize_client_connection,
    }
};

// External


pub fn af_local_listener(
    listen_address: String, 
    handler: HandlerType,
    permission_db: Pool<ConnectionManager<SqliteConnection>>,
    handler_db: Pool<ConnectionManager<SqliteConnection>>,
    connection_handler: fn(Result<LocalSocketStream, Error>, &Pool<ConnectionManager<SqliteConnection>>) -> Result<String, Error>
) -> Result<(), Error>
{
    let listener: LocalSocketListener = LocalSocketListener::bind(listen_address)?;
    
    for conn in listener.incoming() {
        match conn {
            Err(e) => warn!("{}", format!("Listener connection failed: {}", e)),
            Ok(connection) => {
                let pdb = permission_db.clone();
                let hdb = handler_db.clone();
                thread::spawn( move ||
                    {
                        let peer_pid: String = match connection.peer_pid() 
                        {
                            Ok(peer_id) => 
                            {
                                info!("Client connnected, pid<{peer_id}>.");
                                peer_id.to_string()
                            },
                            Err(_e) => 
                            {
                                info!("Client connection, no pid available.");
                                "Unavailable".to_owned()
                            }
                        };
                        match connection_handler(
                            authenticate_authorize(
                                &handler, 
                                &pdb, 
                                connection
                            ),
                            &hdb
                        ) {
                            Ok(log) => info!("{}", log),
                            Err(e) => warn!("{} for pid: {}", e.name().unwrap(), peer_pid)
                        }
                    }
                );
            },
        }
    }

    Ok(())
}

//transaction(authorize(handlers: String, authenticate(pubkey: Vec<u8>, identify(name: String, accept())))