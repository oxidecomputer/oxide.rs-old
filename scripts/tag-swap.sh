#!/bin/bash
#
# Hacky script meant as a workaround until the generated code is generated by progenitor

set -eux

sed -i 's/"operationId": "rack_list",/"operationId": "hardware_racks_get",/g' spec.json
sed -i 's/"operationId": "rack_view",/"operationId": "hardware_racks_get_rack",/g' spec.json
sed -i 's/"operationId": "sled_list",/"operationId": "hardware_sleds_get",/g' spec.json
sed -i 's/"operationId": "sled_view",/"operationId": "hardware_sleds_get_sled",/g' spec.json
sed -i 's/"operationId": "image_global_list",/"operationId": "images_get",/g' spec.json
sed -i 's/"operationId": "image_global_create",/"operationId": "images_post",/g' spec.json
sed -i 's/"operationId": "image_global_delete",/"operationId": "images_delete_image",/g' spec.json
sed -i 's/"operationId": "image_global_view",/"operationId": "images_get_image",/g' spec.json
sed -i 's/"operationId": "ip_pool_list",/"operationId": "ip_pools_get",/g' spec.json
sed -i 's/"operationId": "ip_pool_create",/"operationId": "ip_pools_post",/g' spec.json
sed -i 's/"operationId": "ip_pool_delete",/"operationId": "ip_pools_delete_ip_pool",/g' spec.json
sed -i 's/"operationId": "ip_pool_view",/"operationId": "ip_pools_get_ip_pool",/g' spec.json
sed -i 's/"operationId": "ip_pool_update",/"operationId": "ip_pools_put_ip_pool",/g' spec.json
sed -i 's/"operationId": "ip_pool_range_list",/"operationId": "ip_pool_ranges_get",/g' spec.json
sed -i 's/"operationId": "ip_pool_range_add",/"operationId": "ip_pool_ranges_add",/g' spec.json
sed -i 's/"operationId": "ip_pool_range_remove",/"operationId": "ip_pool_ranges_delete",/g' spec.json
sed -i 's/"operationId": "organization_list",/"operationId": "organizations_get",/g' spec.json
sed -i 's/"operationId": "organization_create",/"operationId": "organizations_post",/g' spec.json
sed -i 's/"operationId": "organization_delete",/"operationId": "organizations_delete_organization",/g' spec.json
sed -i 's/"operationId": "organization_view",/"operationId": "organizations_get_organization",/g' spec.json
sed -i 's/"operationId": "organization_update",/"operationId": "organizations_put_organization",/g' spec.json
sed -i 's/"operationId": "organization_policy_view",/"operationId": "organization_get_policy",/g' spec.json
sed -i 's/"operationId": "organization_policy_update",/"operationId": "organization_put_policy",/g' spec.json
sed -i 's/"operationId": "project_list",/"operationId": "organization_projects_get",/g' spec.json
sed -i 's/"operationId": "project_create",/"operationId": "organization_projects_post",/g' spec.json
sed -i 's/"operationId": "project_delete",/"operationId": "organization_projects_delete_project",/g' spec.json
sed -i 's/"operationId": "project_view",/"operationId": "organization_projects_get_project",/g' spec.json
sed -i 's/"operationId": "project_update",/"operationId": "organization_projects_put_project",/g' spec.json
sed -i 's/"operationId": "disk_list",/"operationId": "project_disks_get",/g' spec.json
sed -i 's/"operationId": "disk_create",/"operationId": "project_disks_post",/g' spec.json
sed -i 's/"operationId": "disk_delete",/"operationId": "project_disks_delete_disk",/g' spec.json
sed -i 's/"operationId": "disk_view",/"operationId": "project_disks_get_disk",/g' spec.json
sed -i 's/"operationId": "image_list",/"operationId": "project_images_get",/g' spec.json
sed -i 's/"operationId": "image_create",/"operationId": "project_images_post",/g' spec.json
sed -i 's/"operationId": "image_delete",/"operationId": "project_images_delete_image",/g' spec.json
sed -i 's/"operationId": "image_view",/"operationId": "project_images_get_image",/g' spec.json
sed -i 's/"operationId": "instance_list",/"operationId": "project_instances_get",/g' spec.json
sed -i 's/"operationId": "instance_create",/"operationId": "project_instances_post",/g' spec.json
sed -i 's/"operationId": "instance_delete",/"operationId": "project_instances_delete_instance",/g' spec.json
sed -i 's/"operationId": "instance_view",/"operationId": "project_instances_get_instance",/g' spec.json
sed -i 's/"operationId": "instance_disk_list",/"operationId": "instance_disks_get",/g' spec.json
sed -i 's/"operationId": "instance_disk_attach",/"operationId": "instance_disks_attach",/g' spec.json
sed -i 's/"operationId": "instance_disk_detach",/"operationId": "instance_disks_detach",/g' spec.json
sed -i 's/"operationId": "instance_migrate",/"operationId": "project_instances_migrate_instance",/g' spec.json
sed -i 's/"operationId": "instance_network_interface_list",/"operationId": "instance_network_interfaces_get",/g' spec.json
sed -i 's/"operationId": "instance_network_interface_create",/"operationId": "instance_network_interfaces_post",/g' spec.json
sed -i 's/"operationId": "instance_network_interface_view",/"operationId": "instance_network_interfaces_get_interface",/g' spec.json
sed -i 's/"operationId": "instance_network_interface_update",/"operationId": "instance_network_interfaces_put_interface",/g' spec.json
sed -i 's/"operationId": "instance_network_interface_delete",/"operationId": "instance_network_interfaces_delete_interface",/g' spec.json
sed -i 's/"operationId": "instance_reboot",/"operationId": "project_instances_instance_reboot",/g' spec.json
sed -i 's/"operationId": "instance_serial_console",/"operationId": "project_instances_instance_serial_get",/g' spec.json
sed -i 's/"operationId": "instance_start",/"operationId": "project_instances_instance_start",/g' spec.json
sed -i 's/"operationId": "instance_stop",/"operationId": "project_instances_instance_stop",/g' spec.json
sed -i 's/"operationId": "project_policy_view",/"operationId": "organization_projects_get_project_policy",/g' spec.json
sed -i 's/"operationId": "project_policy_update",/"operationId": "organization_projects_put_project_policy",/g' spec.json
sed -i 's/"operationId": "snapshot_list",/"operationId": "project_snapshots_get",/g' spec.json
sed -i 's/"operationId": "snapshot_create",/"operationId": "project_snapshots_post",/g' spec.json
sed -i 's/"operationId": "snapshot_delete",/"operationId": "project_snapshots_delete_snapshot",/g' spec.json
sed -i 's/"operationId": "snapshot_view",/"operationId": "project_snapshots_get_snapshot",/g' spec.json
sed -i 's/"operationId": "vpc_list",/"operationId": "project_vpcs_get",/g' spec.json
sed -i 's/"operationId": "vpc_create",/"operationId": "project_vpcs_post",/g' spec.json
sed -i 's/"operationId": "vpc_delete",/"operationId": "project_vpcs_delete_vpc",/g' spec.json
sed -i 's/"operationId": "vpc_view",/"operationId": "project_vpcs_get_vpc",/g' spec.json
sed -i 's/"operationId": "vpc_update",/"operationId": "project_vpcs_put_vpc",/g' spec.json
sed -i 's/"operationId": "vpc_firewall_rules_view",/"operationId": "vpc_firewall_rules_get",/g' spec.json
sed -i 's/"operationId": "vpc_firewall_rules_update",/"operationId": "vpc_firewall_rules_put",/g' spec.json
sed -i 's/"operationId": "vpc_router_list",/"operationId": "vpc_routers_get",/g' spec.json
sed -i 's/"operationId": "vpc_router_create",/"operationId": "vpc_routers_post",/g' spec.json
sed -i 's/"operationId": "vpc_router_delete",/"operationId": "vpc_routers_delete_router",/g' spec.json
sed -i 's/"operationId": "vpc_router_view",/"operationId": "vpc_routers_get_router",/g' spec.json
sed -i 's/"operationId": "vpc_router_update",/"operationId": "vpc_routers_put_router",/g' spec.json
sed -i 's/"operationId": "vpc_router_route_list",/"operationId": "routers_routes_get",/g' spec.json
sed -i 's/"operationId": "vpc_router_route_create",/"operationId": "routers_routes_post",/g' spec.json
sed -i 's/"operationId": "vpc_router_route_delete",/"operationId": "routers_routes_delete_route",/g' spec.json
sed -i 's/"operationId": "vpc_router_route_view",/"operationId": "routers_routes_get_route",/g' spec.json
sed -i 's/"operationId": "vpc_router_route_update",/"operationId": "routers_routes_put_route",/g' spec.json
sed -i 's/"operationId": "vpc_subnet_list",/"operationId": "vpc_subnets_get",/g' spec.json
sed -i 's/"operationId": "vpc_subnet_create",/"operationId": "vpc_subnets_post",/g' spec.json
sed -i 's/"operationId": "vpc_subnet_delete",/"operationId": "vpc_subnets_delete_subnet",/g' spec.json
sed -i 's/"operationId": "vpc_subnet_view",/"operationId": "vpc_subnets_get_subnet",/g' spec.json
sed -i 's/"operationId": "vpc_subnet_update",/"operationId": "vpc_subnets_put_subnet",/g' spec.json
sed -i 's/"operationId": "vpc_subnet_list_network_interfaces",/"operationId": "subnet_network_interfaces_get",/g' spec.json
sed -i 's/"operationId": "policy_view",/"operationId": "policy_get",/g' spec.json
sed -i 's/"operationId": "policy_update",/"operationId": "policy_put",/g' spec.json
sed -i 's/"operationId": "role_list",/"operationId": "roles_get",/g' spec.json
sed -i 's/"operationId": "role_view",/"operationId": "roles_get_role",/g' spec.json
sed -i 's/"operationId": "saga_list",/"operationId": "sagas_get",/g' spec.json
sed -i 's/"operationId": "saga_view",/"operationId": "sagas_get_saga",/g' spec.json
sed -i 's/"operationId": "session_sshkey_list",/"operationId": "sshkeys_get",/g' spec.json
sed -i 's/"operationId": "session_sshkey_create",/"operationId": "sshkeys_post",/g' spec.json
sed -i 's/"operationId": "session_sshkey_delete",/"operationId": "sshkeys_delete_key",/g' spec.json
sed -i 's/"operationId": "session_sshkey_view",/"operationId": "sshkeys_get_key",/g' spec.json
sed -i 's/"operationId": "silo_list",/"operationId": "silos_get",/g' spec.json
sed -i 's/"operationId": "silo_create",/"operationId": "silos_post",/g' spec.json
sed -i 's/"operationId": "silo_delete",/"operationId": "silos_delete_silo",/g' spec.json
sed -i 's/"operationId": "silo_view",/"operationId": "silos_get_silo",/g' spec.json
sed -i 's/"operationId": "silo_identity_provider_list",/"operationId": "silos_get_identity_providers",/g' spec.json
sed -i 's/"operationId": "silo_policy_view",/"operationId": "silos_get_silo_policy",/g' spec.json
sed -i 's/"operationId": "silo_policy_update",/"operationId": "silos_put_silo_policy",/g' spec.json
sed -i 's/"operationId": "silo_identity_provider_view",/"operationId": "silo_saml_idp_create",/g' spec.json
sed -i 's/"operationId": "silo_identity_provider_create",/"operationId": "silo_saml_idp_fetch",/g' spec.json
sed -i 's/"operationId": "user_list",/"operationId": "silo_users_get",/g' spec.json