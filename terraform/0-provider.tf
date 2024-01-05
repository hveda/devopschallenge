# Google Cloud Platform Provider
# https://registry.terraform.io/providers/hashicorp/google/latest/docs
provider "google" {
  region = local.region
}

terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 3.66"
    }
  }
}