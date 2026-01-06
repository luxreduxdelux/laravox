/*
* Copyright (c) 2025 luxreduxdelux
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*
* 1. Redistributions of source code must retain the above copyright notice,
* this list of conditions and the following disclaimer.
*
* 2. Redistributions in binary form must reproduce the above copyright notice,
* this list of conditions and the following disclaimer in the documentation
* and/or other materials provided with the distribution.
*
* Subject to the terms and conditions of this license, each copyright holder
* and contributor hereby grants to those receiving rights under this license
* a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable
* (except for failure to satisfy the conditions of this license) patent license
* to make, have made, use, offer to sell, sell, import, and otherwise transfer
* this software, where such license applies only to those patent claims, already
* acquired or hereafter acquired, licensable by such copyright holder or
* contributor that are necessarily infringed by:
*
* (a) their Contribution(s) (the licensed copyrights of copyright holders and
* non-copyrightable additions of contributors, in source or binary form) alone;
* or
*
* (b) combination of their Contribution(s) with the work of authorship to which
* such Contribution(s) was added by such copyright holder or contributor, if,
* at the time the Contribution is added, such addition causes such combination
* to be necessarily infringed. The patent license shall not apply to any other
* combinations which include the Contribution.
*
* Except as expressly stated above, no rights or licenses from any copyright
* holder or contributor is granted under this license, whether expressly, by
* implication, estoppel or otherwise.
*
* DISCLAIMER
*
* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
* AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
* IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE
* FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
* DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
* SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
* CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
* OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

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
        result(
            name = "server",
            info = "Server resource.",
            kind(user_data(name = "Server"))
        )
    )]
    fn new_server(
        _: &mlua::Lua,
        (address_a, address_b, address_c, address_d, port): (u8, u8, u8, u8, u16),
    ) -> mlua::Result<Self> {
        let server = RenetServer::new(ConnectionConfig::default());

        let address = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(address_a, address_b, address_c, address_d)),
            port,
        );

        println!("add: {address:?}");

        let socket: UdpSocket = UdpSocket::bind(address)?;
        let server_config = ServerConfig {
            current_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap(),
            max_clients: 4,
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
    ) -> mlua::Result<(Vec<LuaValue>, Vec<u64>, Vec<u64>)> {
        let delta = Duration::from_millis(delta);
        this.server.update(delta);
        this.transport.update(delta, &mut this.server);

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
                let message: serde_value::Value = rmp_serde::from_slice(&message).unwrap();
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
        let message = rmp_serde::to_vec(&message).unwrap();

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
        let message = rmp_serde::to_vec(&message).unwrap();

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
        let message = rmp_serde::to_vec(&message).unwrap();

        this.server
            .broadcast_message_except(client, DefaultChannel::ReliableOrdered, message);

        Ok(())
    }

    #[method(from = "server", info = "Disconnect a specific client.")]
    fn disconnect(_: &mlua::Lua, this: &mut Self, client: u64) -> mlua::Result<()> {
        this.server.disconnect(client);
        this.transport.send_packets(&mut this.server);

        Ok(())
    }

    #[method(from = "server", info = "Disconnect every client.")]
    fn disconnect_all(_: &mlua::Lua, this: &mut Self, _: ()) -> mlua::Result<()> {
        this.server.disconnect_all();
        this.transport.send_packets(&mut this.server);

        Ok(())
    }
}

#[rustfmt::skip]
impl mlua::UserData for Server {
    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method_mut("update",            Self::update);
        method.add_method_mut("set",               Self::set);
        method.add_method_mut("set_client",        Self::set_client);
        method.add_method_mut("set_client_except", Self::set_client_except);
        method.add_method_mut("disconnect",        Self::disconnect);
        method.add_method_mut("disconnect_all",    Self::disconnect_all);
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
        result(
            name = "client",
            info = "Client resource.",
            kind(user_data(name = "Client"))
        )
    )]
    fn new_client(
        _: &mlua::Lua,
        (address_a, address_b, address_c, address_d, port): (u8, u8, u8, u8, u16),
    ) -> mlua::Result<Self> {
        let client = RenetClient::new(ConnectionConfig::default());

        let address = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(address_a, address_b, address_c, address_d)),
            port,
        );

        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let authentication = ClientAuthentication::Unsecure {
            server_addr: address,
            client_id: 0,
            user_data: None,
            protocol_id: 0,
        };

        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

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
    fn update(lua: &mlua::Lua, this: &mut Self, delta: u64) -> mlua::Result<Vec<LuaValue>> {
        let delta = Duration::from_millis(delta);
        this.client.update(delta);
        this.transport.update(delta, &mut this.client).unwrap();

        let mut message_list = Vec::new();

        if this.client.is_connected() {
            while let Some(message) = this.client.receive_message(DefaultChannel::ReliableOrdered) {
                let message: serde_value::Value = rmp_serde::from_slice(&message).unwrap();
                let message = lua.to_value(&message)?;

                message_list.push(lua.to_value(&message)?);
            }

            this.transport.send_packets(&mut this.client).unwrap();
        }

        Ok(message_list)
    }

    #[method(from = "client", info = "Get the round-trip time to the server.")]
    fn get_round_trip_time(_: &mlua::Lua, this: &mut Self, _: ()) -> mlua::Result<f64> {
        Ok(this.client.rtt())
    }

    #[method(
        from = "Client",
        info = "Send a message to the server.",
        parameter(name = "message", info = "Message to send.", kind = "table")
    )]
    fn set(lua: &mlua::Lua, this: &mut Self, message: mlua::Value) -> mlua::Result<()> {
        if this.client.is_connected() {
            let message: serde_value::Value = lua.from_value(message)?;
            let message = rmp_serde::to_vec(&message).unwrap();

            this.client
                .send_message(DefaultChannel::ReliableOrdered, message);
        }

        Ok(())
    }

    #[method(from = "client", info = "Disconnect from the server.")]
    fn disconnect(_: &mlua::Lua, this: &mut Self, _: ()) -> mlua::Result<()> {
        // NOTE: this.client.disconnect() does not appear to work, only transport does?
        this.transport.disconnect();

        Ok(())
    }

    #[method(from = "client", info = "Get the connection status to the server.")]
    fn get_status(_: &mlua::Lua, this: &mut Self, _: ()) -> mlua::Result<(i32, Option<String>)> {
        if this.client.is_connected() {
            return Ok((0, None));
        } else if this.client.is_connecting() {
            return Ok((1, None));
        }

        Ok((2, this.client.disconnect_reason().map(|x| x.to_string())))
    }
}

#[rustfmt::skip]
impl mlua::UserData for Client {
    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method_mut("update",              Self::update);
        method.add_method_mut("get_round_trip_time", Self::get_round_trip_time);
        method.add_method_mut("set",                 Self::set);
        method.add_method_mut("disconnect",          Self::disconnect);
        method.add_method_mut("get_status",          Self::get_status);
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.transport.disconnect();
    }
}
