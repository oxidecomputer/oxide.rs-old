use std::str::FromStr;

use inflector::cases::pascalcase::to_pascal_case;
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
    assert_eq!(variants, vec!["ip", "ip_net", "subnet", "vpc"]);

    let ip = crate::types::IpNet::V4(crate::types::Ipv4Net(
        ipnetwork::Ipv4Network::new(std::net::Ipv4Addr::new(172, 30, 0, 0), 22).unwrap(),
    ));
    let ip_str = format!("{}", ip);
    assert_eq!(ip_str, "172.30.0.0/22");

    route_destination = crate::types::RouteDestination::IpNet(ip);
    route_destination_str = format!("{}", route_destination);
    assert_eq!(route_destination_str, "ip_net=172.30.0.0/22");

    route_destination_from_str =
        crate::types::RouteDestination::from_str("ip_net=172.30.0.0/22").unwrap();
    assert_eq!(route_destination_from_str, route_destination);
}

#[test]
fn test_disk_source() {
    let mut disk_source = crate::types::DiskSource::Snapshot("some-string-uuid".to_string());
    let mut disk_source_str = format!("{}", disk_source);
    assert_eq!(disk_source_str, "snapshot=some-string-uuid");

    let mut disk_source_from_str =
        crate::types::DiskSource::from_str("snapshot=some-string-uuid").unwrap();
    assert_eq!(disk_source_from_str, disk_source);

    disk_source = crate::types::DiskSource::Image("some-image-string".to_string());
    disk_source_str = format!("{}", disk_source);
    assert_eq!(disk_source_str, "image=some-image-string");

    disk_source_from_str = crate::types::DiskSource::from_str("image=some-image-string").unwrap();
    assert_eq!(disk_source_from_str, disk_source);

    let variants = crate::types::DiskSource::variants();
    assert_eq!(variants.len(), 4);
    assert_eq!(variants, vec!["blank", "global_image", "image", "snapshot"]);

    disk_source = crate::types::DiskSource::GlobalImage("some-image-string".to_string());
    disk_source_str = format!("{}", disk_source);
    assert_eq!(disk_source_str, "global_image=some-image-string");

    disk_source_from_str =
        crate::types::DiskSource::from_str("global_image=some-image-string").unwrap();
    assert_eq!(disk_source_from_str, disk_source);

    disk_source = crate::types::DiskSource::Blank(432);
    disk_source_str = format!("{}", disk_source);
    assert_eq!(disk_source_str, "blank=432");

    disk_source_from_str = crate::types::DiskSource::from_str("blank=432").unwrap();
    assert_eq!(disk_source_from_str, disk_source);
}

#[test]
fn test_disk_source_type() {
    let mut disk_source_type = crate::types::DiskSourceType::Snapshot;
    let mut disk_source_type_str = format!("{}", disk_source_type);
    assert_eq!(disk_source_type_str, "Snapshot");

    let mut disk_source_type_from_str = crate::types::DiskSourceType::from_str("Snapshot").unwrap();
    assert_eq!(disk_source_type_from_str, disk_source_type);

    disk_source_type = crate::types::DiskSourceType::GlobalImage;
    disk_source_type_str = format!("{}", disk_source_type);
    assert_eq!(disk_source_type_str, "GlobalImage");

    let pascal = to_pascal_case("global_image");
    assert_eq!(pascal, "GlobalImage");

    disk_source_type_from_str = crate::types::DiskSourceType::from_str(&pascal).unwrap();
    assert_eq!(disk_source_type_from_str, disk_source_type);
}
