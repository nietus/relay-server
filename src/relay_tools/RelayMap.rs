use std::fmt::format;
use std::net::SocketAddrV4;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use super::peer_data::{PeerData, PublicKey};

pub const TIME_TO_LIVE: u128 = 600_000; // 10 min
pub const MAX_RELAY_COUNT: usize = 3_000;

pub struct RelayMap {
    pub inner: std::collections::HashMap<PublicKey, PeerData>,
}

impl RelayMap {
    pub fn new() -> Self {
        Self {
            inner: std::collections::HashMap::new(),
        }
    }

    pub fn bind_peer(
        &mut self,
        public_key: PublicKey,
        peer_addr: SocketAddrV4,
    ) -> Result<(), String> {
        if self.inner.len() >= MAX_RELAY_COUNT {
            return Err("Relay map está cheio".into());
        }

        // Check if this is the placeholder address (127.0.0.9:9)
        let is_placeholder = peer_addr.ip().octets() == [127, 0, 0, 9] && peer_addr.port() == 9;
        
        // Check if we already have this peer with a valid address
        if let Some(existing_peer) = self.inner.get(&public_key) {
            let existing_is_placeholder = existing_peer.peer_addr.ip().octets() == [127, 0, 0, 9] && existing_peer.peer_addr.port() == 9;
            
            if is_placeholder && !existing_is_placeholder {
                // Don't replace a valid address with a placeholder
                println!(
                    "Keeping existing valid address {} for peer {} (rejecting placeholder)",
                    existing_peer.peer_addr, public_key
                );
                
                // Just update the timestamp
                if let Some(peer_data) = self.inner.get_mut(&public_key) {
                    peer_data.discovery_time = now_ms();
                }
                
                return Ok(());
            }
        }
        
        // Create new peer data
        let peer_data = PeerData::new(public_key.clone(), peer_addr);
        println!(
            "Binding peer : {}  to {}",
            peer_data.public_key, peer_data.peer_addr
        );
        self.inner.insert(public_key, peer_data);

        Ok(())
    }

    pub fn get_mut(&mut self, id: &PublicKey) -> Option<&mut PeerData> {
        self.inner.get_mut(id)
    }

    pub fn get(&self, id: &PublicKey) -> Option<&PeerData> {
        self.inner.get(id)
    }

    pub fn garbage_collect(&mut self) {
        let now = now_ms();
        self.inner
            .retain(|_, data| now - data.discovery_time <= TIME_TO_LIVE);
    }

    pub fn has_peer(&self, p_key: &PublicKey) -> bool {
        self.inner.contains_key(p_key)
    }

    pub fn get_peerData_mut(&mut self, p_key: &PublicKey) -> &mut PeerData {
        self.inner.get_mut(p_key).unwrap()
    }

    pub fn reset_peer_time(&mut self, p_key: &PublicKey) {
        if let Some(peer_data) = self.inner.get_mut(p_key) {
            peer_data.discovery_time = now_ms();
        }
    }
}

pub fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
