// Small helpers (formatting, time)
pub fn short_addr(addr: &str) -> String {
    if addr.len() > 12 { format!("{}...{}", &addr[..6], &addr[addr.len()-4..]) } else { addr.to_string() }
}
