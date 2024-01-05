locals {
  region               = "asia-southeast2"
  project_name         = "devops-poc-id"
  projects_api         = "container.googleapis.com"
  initial_node_count   = 1
  cluster_name         = "default-cluster"
  machine_type         = "e2-micro"
  node_count           = 1
  secondary_ip_ranges  = {
    "pod-ip-range"      = "10.0.0.0/14",
    "services-ip-range" = "10.4.0.0/19"
  }
}
