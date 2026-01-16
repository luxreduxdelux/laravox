use crate::module::general::*;
use engine_macro::*;

//================================================================

use mlua::prelude::*;
use renet::ConnectionConfig;
use renet::DefaultChannel;
use renet::RenetClient;
use renet::RenetServer;
use renet::ServerEvent;
use renet_netcode::ClientAuthentication;
use renet_netcode::NetcodeClientTransport;
use renet_netcode::NetcodeServerTransport;
use renet_netcode::ServerAuthentication;
use renet_netcode::ServerConfig;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::time::Duration;
use std::time::SystemTime;

//================================================================

#[rustfmt::skip]
#[module(name = "network", info = "Network API.")]
pub fn set_global(lua: &mlua::Lua, global: &mlua::Table) -> anyhow::Result<()> {
    let network = lua.create_table()?;

    network.set("new_server", lua.create_function(self::Server::new_server)?)?;
    network.set("new_client", lua.create_function(self::Client::new_client)?)?;

    global.set("network", network)?;

    Ok(())
}

//================================================================

#[class(info = "Server class.")]
struct Server {
    server: RenetServer,
    transport: NetcodeServerTransport,
}

impl Server {
    #[function(
        from = "network",
        info = "Create a new server.",
        parameter(
            name = "address_a",
            info = "Segment 1 of IPV4 address.",
            kind = "number"
        ),
        parameter(
            name = "address_b",
            info = "Segment 2 of IPV4 address.",
            kind = "number"
        ),
        parameter(
            name = "address_c",
            info = "Segment 3 of IPV4 address.",
            kind = "number"
        ),
        parameter(
            name = "address_d",
            info = "Segment 4 of IPV4 address.",
            kind = "number"
        ),
        parameter(name = "port", info = "Address port.", kind = "number"),
        parameter(name = "client_count", info = "Maximum client count.", kind = "number"),
        result(
            name = "server",
            info = "Server resource.",
            kind(user_data(name = "Server"))
        )
    )]
    fn new_server(
        _: &mlua::Lua,
        (address_a, address_b, address_c, address_d, port, client_count): (
            u8,
            u8,
            u8,
            u8,
            u16,
            usize,
        ),
    ) -> mlua::Result<Self> {
        let server = RenetServer::new(ConnectionConfig::default());
        let address = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(address_a, address_b, address_c, address_d)),
            port,
        );

        let socket: UdpSocket = UdpSocket::bind(address)?;
        let server_config = ServerConfig {
            current_time: map_error(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH))?,
            max_clients: client_count,
            protocol_id: 0,
            public_addresses: vec![address],
            authentication: ServerAuthentication::Unsecure,
        };
        let transport = NetcodeServerTransport::new(server_config, socket)?;

        Ok(Self { server, transport })
    }

    #[method(
        from = "Server",
        info = "Update the server.",
        parameter(name = "delta", info = "Time delta.", kind = "number"),
        result(
            name = "message_list",
            info = "Table array, containing every message.",
            kind = "table"
        ),
        result(
            name = "enter_list",
            info = "Table array, containing every new connection.",
            kind = "table"
        ),
        result(
            name = "leave_list",
            info = "Table array, containing every new disconnection.",
            kind = "table"
        )
    )]
    fn update(
        lua: &mlua::Lua,
        this: &mut Self,
        delta: u64,
    ) -> mlua::Result<(Vec<mlua::Value>, Vec<u64>, Vec<u64>)> {
        let delta = Duration::from_millis(delta);
        this.server.update(delta);
        map_error(this.transport.update(delta, &mut this.server))?;

        let mut message_list = Vec::new();
        let mut enter_list = Vec::new();
        let mut leave_list = Vec::new();

        while let Some(event) = this.server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    println!("Client connect {client_id}");

                    enter_list.push(client_id);
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Client disconnect {client_id}");
                    leave_list.push(client_id);
                }
            }
        }

        for client_id in this.server.clients_id() {
            while let Some(message) = this
                .server
                .receive_message(client_id, DefaultChannel::ReliableOrdered)
            {
                let message: serde_value::Value = map_error(rmp_serde::from_slice(&message))?;
                let message = lua.to_value(&message)?;

                message_list.push(lua.to_value(&(client_id, message))?);
            }
        }

        this.transport.send_packets(&mut this.server);

        Ok((message_list, enter_list, leave_list))
    }

    #[method(
        from = "Server",
        info = "Send a message to every client.",
        parameter(name = "message", info = "Message to send.", kind = "table")
    )]
    fn set(lua: &mlua::Lua, this: &mut Self, message: mlua::Value) -> mlua::Result<()> {
        let message: serde_value::Value = lua.from_value(message)?;
        let message = map_error(rmp_serde::to_vec_named(&message))?;

        this.server
            .broadcast_message(DefaultChannel::ReliableOrdered, message);

        Ok(())
    }

    #[method(
        from = "Server",
        info = "Send a message to a specific client.",
        parameter(name = "message", info = "Message to send.", kind = "table"),
        parameter(name = "client", info = "Specific client.", kind = "number")
    )]
    fn set_client(
        lua: &mlua::Lua,
        this: &mut Self,
        (message, client): (mlua::Value, u64),
    ) -> mlua::Result<()> {
        let message: serde_value::Value = lua.from_value(message)?;
        let message = map_error(rmp_serde::to_vec_named(&message))?;

        this.server
            .send_message(client, DefaultChannel::ReliableOrdered, message);

        Ok(())
    }

    #[method(
        from = "Server",
        info = "Send a message to every client, except a specific client.",
        parameter(name = "message", info = "Message to send.", kind = "table"),
        parameter(name = "client", info = "Specific client.", kind = "number")
    )]
    fn set_client_except(
        lua: &mlua::Lua,
        this: &mut Self,
        (message, client): (mlua::Value, u64),
    ) -> mlua::Result<()> {
        let message: serde_value::Value = lua.from_value(message)?;
        let message = map_error(rmp_serde::to_vec_named(&message))?;

        this.server
            .broadcast_message_except(client, DefaultChannel::ReliableOrdered, message);

        Ok(())
    }

    #[method(from = "Server", info = "Disconnect a specific client.")]
    fn disconnect(_: &mlua::Lua, this: &mut Self, client: u64) -> mlua::Result<()> {
        this.server.disconnect(client);

        Ok(())
    }

    #[method(from = "Server", info = "Disconnect every client.")]
    fn disconnect_all(_: &mlua::Lua, this: &mut Self, _: ()) -> mlua::Result<()> {
        this.transport.disconnect_all(&mut this.server);

        Ok(())
    }

    #[method(from = "Server", info = "Get the user-data for a specific client.")]
    fn get_client_user_data(
        lua: &mlua::Lua,
        this: &mut Self,
        client: u64,
    ) -> mlua::Result<Option<mlua::Value>> {
        let user_data = if let Some(user_data) = this.transport.user_data(client) {
            let message: serde_value::Value = map_error(rmp_serde::from_slice(&user_data))?;
            let message = lua.to_value(&message)?;

            Some(message)
        } else {
            None
        };

        Ok(user_data)
    }
}

#[rustfmt::skip]
impl mlua::UserData for Server {
    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method_mut("update",               Self::update);
        method.add_method_mut("set",                  Self::set);
        method.add_method_mut("set_client",           Self::set_client);
        method.add_method_mut("set_client_except",    Self::set_client_except);
        method.add_method_mut("disconnect",           Self::disconnect);
        method.add_method_mut("disconnect_all",       Self::disconnect_all);
        method.add_method_mut("get_client_user_data", Self::get_client_user_data);
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.transport.disconnect_all(&mut self.server);
    }
}

//================================================================

#[class(info = "Client class.")]
struct Client {
    client: RenetClient,
    transport: NetcodeClientTransport,
}

impl Client {
    #[function(
        from = "network",
        info = "Create a new client.",
        parameter(
            name = "address_a",
            info = "Segment 1 of IPV4 address.",
            kind = "number"
        ),
        parameter(
            name = "address_b",
            info = "Segment 2 of IPV4 address.",
            kind = "number"
        ),
        parameter(
            name = "address_c",
            info = "Segment 3 of IPV4 address.",
            kind = "number"
        ),
        parameter(
            name = "address_d",
            info = "Segment 4 of IPV4 address.",
            kind = "number"
        ),
        parameter(name = "port", info = "Address port.", kind = "number"),
        parameter(name = "user_data", info = "User data.", kind = "table"),
        result(
            name = "client",
            info = "Client resource.",
            kind(user_data(name = "Client"))
        )
    )]
    fn new_client(
        lua: &mlua::Lua,
        (address_a, address_b, address_c, address_d, port, user_data): (
            u8,
            u8,
            u8,
            u8,
            u16,
            Option<mlua::Value>,
        ),
    ) -> mlua::Result<Self> {
        let client = RenetClient::new(ConnectionConfig::default());

        let address = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(address_a, address_b, address_c, address_d)),
            port,
        );

        let user_data = if let Some(user_data) = user_data {
            let user_data: serde_value::Value = lua.from_value(user_data)?;
            let mut user_data = map_error(rmp_serde::to_vec(&user_data))?;
            user_data.resize(256, 0);

            Some(user_data.try_into().unwrap())
        } else {
            None
        };

        let socket = UdpSocket::bind("0.0.0.0:0")?;
        let current_time = map_error(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH))?;
        let authentication = ClientAuthentication::Unsecure {
            server_addr: address,
            client_id: 0,
            user_data,
            protocol_id: 0,
        };

        let transport = map_error(NetcodeClientTransport::new(
            current_time,
            authentication,
            socket,
        ))?;

        Ok(Self { client, transport })
    }

    #[method(
        from = "Client",
        info = "Update the client.",
        parameter(name = "delta", info = "Time delta.", kind = "number"),
        result(
            name = "message_list",
            info = "Table array, containing every message.",
            kind = "table"
        )
    )]
    fn update(lua: &mlua::Lua, this: &mut Self, delta: u64) -> mlua::Result<Vec<mlua::Value>> {
        let delta = Duration::from_millis(delta);
        this.client.update(delta);
        map_error(this.transport.update(delta, &mut this.client))?;

        let mut message_list = Vec::new();

        if this.client.is_connected() {
            while let Some(message) = this.client.receive_message(DefaultChannel::ReliableOrdered) {
                let message: serde_value::Value = map_error(rmp_serde::from_slice(&message))?;
                let message = lua.to_value(&message)?;

                message_list.push(lua.to_value(&message)?);
            }
        }

        map_error(this.transport.send_packets(&mut this.client))?;

        Ok(message_list)
    }

    #[method(
        from = "Client",
        info = "Send a message to the server.",
        parameter(name = "message", info = "Message to send.", kind = "table")
    )]
    fn set(lua: &mlua::Lua, this: &mut Self, message: mlua::Value) -> mlua::Result<()> {
        if this.client.is_connected() {
            let message: serde_value::Value = lua.from_value(message)?;
            let message = map_error(rmp_serde::to_vec(&message))?;

            this.client
                .send_message(DefaultChannel::ReliableOrdered, message);
        }

        Ok(())
    }

    #[method(from = "Client", info = "Disconnect from the server.")]
    fn disconnect(_: &mlua::Lua, this: &mut Self, _: ()) -> mlua::Result<()> {
        // NOTE: this.client.disconnect() does not appear to work, only transport does?
        this.transport.disconnect();

        Ok(())
    }

    #[method(
        from = "Client",
        info = "Get the connection status to the server.",
        result(
            name = "status",
            info = "Connection status.",
            kind(user_data(name = "ConnectionStatus"))
        )
    )]
    fn get_status(_: &mlua::Lua, this: &mut Self, _: ()) -> mlua::Result<usize> {
        if this.client.is_connected() {
            return Ok(0);
        } else if this.client.is_connecting() {
            return Ok(1);
        }

        Ok(2)
    }

    #[method(
        from = "Client",
        info = "Get the client's unique identifier.",
        result(name = "identifier", info = "Client identifier.", kind = "number")
    )]
    fn get_identifier(_: &mlua::Lua, this: &mut Self, _: ()) -> mlua::Result<u64> {
        Ok(this.transport.client_id())
    }
}

#[rustfmt::skip]
impl mlua::UserData for Client {
    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method_mut("update",              Self::update);
        method.add_method_mut("set",                 Self::set);
        method.add_method_mut("disconnect",          Self::disconnect);
        method.add_method_mut("get_status",          Self::get_status);
        method.add_method_mut("get_identifier",      Self::get_identifier);
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.transport.disconnect();
    }
}
