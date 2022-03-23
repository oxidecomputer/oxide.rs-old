use std::str::FromStr;

use pretty_assertions::assert_eq;

#[test]
fn test_route_target() {
    let mut route_target = crate::types::RouteTarget::Instance("test".to_string());
    let mut route_target_str = format!("{}", route_target);
    assert_eq!(route_target_str, "instance=test");

    let mut route_target_from_str = crate::types::RouteTarget::from_str("instance=test").unwrap();
    assert_eq!(route_target_from_str, route_target);

    route_target = crate::types::RouteTarget::Ip("192.1.13.2".to_string());
    route_target_str = format!("{}", route_target);
    assert_eq!(route_target_str, "ip=192.1.13.2");

    route_target_from_str = crate::types::RouteTarget::from_str("ip=192.1.13.2").unwrap();
    assert_eq!(route_target_from_str, route_target);

    route_target = crate::types::RouteTarget::Subnet("192.1.13.2".to_string());
    route_target_str = format!("{}", route_target);
    assert_eq!(route_target_str, "subnet=192.1.13.2");

    route_target_from_str = crate::types::RouteTarget::from_str("subnet=192.1.13.2").unwrap();
    assert_eq!(route_target_from_str, route_target);

    route_target = crate::types::RouteTarget::InternetGateway("192.1.13.2".to_string());
    route_target_str = format!("{}", route_target);
    assert_eq!(route_target_str, "inetgw=192.1.13.2");

    route_target_from_str = crate::types::RouteTarget::from_str("inetgw=192.1.13.2").unwrap();
    assert_eq!(route_target_from_str, route_target);

    let variants = crate::types::RouteTarget::variants();
    assert_eq!(variants.len(), 5);
    assert_eq!(
        variants,
        vec![
            "instance".to_string(),
            "inetgw".to_string(),
            "ip".to_string(),
            "subnet".to_string(),
            "vpc".to_string(),
        ]
    );
}

#[test]
fn test_route_destination() {
    let mut route_destination = crate::types::RouteDestination::Vpc("test".to_string());
    let mut route_destination_str = format!("{}", route_destination);
    assert_eq!(route_destination_str, "vpc=test");

    let mut route_destination_from_str =
        crate::types::RouteDestination::from_str("vpc=test").unwrap();
    assert_eq!(route_destination_from_str, route_destination);

    route_destination = crate::types::RouteDestination::Ip("192.1.13.2".to_string());
    route_destination_str = format!("{}", route_destination);
    assert_eq!(route_destination_str, "ip=192.1.13.2");

    route_destination_from_str = crate::types::RouteDestination::from_str("ip=192.1.13.2").unwrap();
    assert_eq!(route_destination_from_str, route_destination);

    route_destination = crate::types::RouteDestination::Subnet("192.1.13.2".to_string());
    route_destination_str = format!("{}", route_destination);
    assert_eq!(route_destination_str, "subnet=192.1.13.2");

    route_destination_from_str =
        crate::types::RouteDestination::from_str("subnet=192.1.13.2").unwrap();
    assert_eq!(route_destination_from_str, route_destination);

    let variants = crate::types::RouteDestination::variants();
    assert_eq!(variants.len(), 4);
    assert_eq!(variants, vec!["ip", "ipnet", "subnet", "vpc"]);

    let ip = crate::types::IpNet::V4(crate::types::Ipv4Net(
        ipnetwork::Ipv4Network::new(std::net::Ipv4Addr::new(172, 30, 0, 0), 22).unwrap(),
    ));
    let ip_str = format!("{}", ip);
    assert_eq!(ip_str, "172.30.0.0/22");

    route_destination = crate::types::RouteDestination::IpNet(ip);
    route_destination_str = format!("{}", route_destination);
    assert_eq!(route_destination_str, "ipnet=172.30.0.0/22");

    route_destination_from_str =
        crate::types::RouteDestination::from_str("ipnet=172.30.0.0/22").unwrap();
    assert_eq!(route_destination_from_str, route_destination);
}
