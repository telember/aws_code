use aws_sdk_ec2::{Client, Error};
use aws_config::meta::region::RegionProviderChain;
use std::collections::HashSet;


#[derive(Debug)]
struct UnusedSecurityGroup {
    id: String,
    name: String,
    vpc_id: String,
}

async fn get_all_security_groups(client: &Client) -> Result<Vec<UnusedSecurityGroup>, Error> {
    let resp = client.describe_security_groups().send().await?;
    let mut security_groups = Vec::new();
    
    for group in resp.security_groups() {
        security_groups.push(UnusedSecurityGroup {
            id: group.group_id().unwrap_or_default().to_string(),
            name: group.group_name().unwrap_or_default().to_string(),
            vpc_id: group.vpc_id().unwrap_or_default().to_string(),
        });
    }
    
    Ok(security_groups)
}

async fn get_used_security_groups(client: &Client) -> Result<HashSet<String>, Error> {
    let mut used_sgs = HashSet::new();
    // Check EC2 instances
    let resp = client.describe_instances().send().await?;
    for reservation in resp.reservations() {
        for instance in reservation.instances() {
            for sg in instance.security_groups() {
                used_sgs.insert(sg.group_id().unwrap_or_default().to_string());
            }
        }
    }
    
    // Check ENIs
    let resp = client.describe_network_interfaces().send().await?;
    for eni in resp.network_interfaces() {
        for group in eni.groups() {
            used_sgs.insert(group.group_id().unwrap_or_default().to_string());
        }
    }
    
    Ok(used_sgs)
}

pub async fn scan() {
    let region_provider = RegionProviderChain::default_provider();
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    
    println!("Scanning for unused security groups...");
    
    let all_security_groups: Vec<UnusedSecurityGroup> = get_all_security_groups(&client).await.unwrap();
    let used_security_groups = get_used_security_groups(&client).await.unwrap();
    
    let unused_groups: Vec<&UnusedSecurityGroup> = all_security_groups
        .iter()
        .filter(|sg| !used_security_groups.contains(&sg.id))
        .collect();
    
    println!("\nUnused Security Groups:");
    println!("{:<20} {:<30} {:<20}", 
        "Security Group ID", 
        "Name", 
        "VPC ID"
    );
    println!("{}", "-".repeat(70));
    
    for sg in unused_groups {
        println!("{:<20} {:<30} {:<20}",
            sg.id,
            sg.name,
            sg.vpc_id
        );
    }
    
}

