use std::net::SocketAddr;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::PathBuf;

use pluginop::api::ToPluginizableConnection;
use pluginop::PluginizableConnection;

fn exports_func(
    _: &mut pluginop::Store,
    _: &pluginop::FunctionEnv<pluginop::plugin::Env<core_quiche::Connection>>,
) -> pluginop::Exports {
    pluginop::Exports::new()
}

pub struct Connection(Box<PluginizableConnection<core_quiche::Connection>>);

#[inline]
pub fn accept(
    scid: &ConnectionId,
    odcid: Option<&ConnectionId>,
    local: SocketAddr,
    peer: SocketAddr,
    config: &mut Config,
) -> Result<Connection> {
    let conn = core_quiche::accept(scid, odcid, local, peer, config)?;

    Ok(Connection::new_with_core_quiche(conn))
}

#[inline]
pub fn connect(
    server_name: Option<&str>,
    scid: &ConnectionId,
    local: SocketAddr,
    peer: SocketAddr,
    config: &mut Config,
) -> Result<Connection> {
    let conn = core_quiche::connect(server_name, scid, local, peer, config)?;

    Ok(Connection::new_with_core_quiche(conn))
}

impl Connection {
    fn new_with_core_quiche(conn: core_quiche::Connection) -> Connection {
        let mut pc = PluginizableConnection::new_pluginizable_connection(exports_func, conn);

        let pc_ptr = &mut *pc as *mut _;
        pc.get_conn_mut().set_pluginizable_connection(pc_ptr);
        pc.get_ph_mut().set_pluginizable_connection(pc_ptr);
        Connection(pc)
    }

    /// Insert a plugin.
    pub fn insert_plugin(&mut self, plugin_fname: &PathBuf) -> bool {
        self.0.ph.insert_plugin(plugin_fname)
    }
}

impl Deref for Connection {
    type Target = core_quiche::Connection;

    fn deref(&self) -> &Self::Target {
        self.0.get_conn()
    }
}

impl DerefMut for Connection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.get_conn_mut()
    }
}

// Reexport quiche structures, such as a quiche-enabled application can use
// Core QUIC with minimal changes.
pub use core_quiche::h3;
pub use core_quiche::negotiate_version;
pub use core_quiche::retry;
pub use core_quiche::version_is_supported;
pub use core_quiche::Config;
pub use core_quiche::ConnectionId;
pub use core_quiche::Error;
pub use core_quiche::Header;
pub use core_quiche::PathEvent;
pub use core_quiche::PathStats;
pub use core_quiche::RecvInfo;
pub use core_quiche::Result;
pub use core_quiche::SendInfo;
pub use core_quiche::Shutdown;
pub use core_quiche::Type;
pub use core_quiche::MAX_CONN_ID_LEN;
pub use core_quiche::PROTOCOL_VERSION;
