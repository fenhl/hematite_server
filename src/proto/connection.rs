use std::io;
use std::io::prelude::*;
use std::sync::Arc;

use packet::{handshake, play, status, login, PacketWrite};

/// A source of packets.
pub trait ServerPacketSource {
    fn read_handshake(&mut self) -> io::Result<handshake::PacketArc>;
    fn read_play(&mut self) -> io::Result<play::serverbound::PacketArc>;
    fn read_status(&mut self) -> io::Result<status::serverbound::PacketArc>;
    fn read_login(&mut self) -> io::Result<login::serverbound::PacketArc>;
}

/// A sink of packets.
pub trait PacketSink {
    fn write(&mut self, Arc<PacketWrite>) -> io::Result<()>;
}
