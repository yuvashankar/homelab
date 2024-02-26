terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "5.38.0"
    }
  }
}

provider "aws" {
  region                   = "ca-central-1"
  shared_credentials_files = ["/home/vscode/.aws/config"]
  shared_config_files      = ["/home/vscode/.aws/credentials"]
  profile                  = "default"
}


resource "aws_iam_group" "dns_group" {
  name = "dns_group"
}
