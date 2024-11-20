enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[cfg(test)]
mod tests {
    use crate::ip_address::IpAddrKind;

    #[test]
    fn test_ip_address() {
        let home = IpAddrKind::V4(127, 0, 0, 1);
        let loopback = IpAddrKind::V6(String::from("::1"));
    }
}
