locals {
  region               = "asia-southeast2"
  zone                 = "asia-southeast2-c"
  project_name         = "devops-poc-id"
  projects_api         = "container.googleapis.com"
  initial_node_count   = 1
  cluster_name         = "gke-default"
  machine_type         = "e2-small"
  network              = "projects/devops-poc-id/global/networks/gke-default"
  node_count           = 1
  secondary_ip_ranges  = {
    "${local.cluster_name}-pod-range"      = "10.80.0.0/14",
    "${local.cluster_name}-services-range" = "10.84.0.0/19"
  }
}
