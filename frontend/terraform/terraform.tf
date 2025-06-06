terraform {
  backend "s3" {
    region         = "eu-west-1"
    bucket         = "oliver-bilbie-tf-state-bucket"
    key            = "aoc-solver/terraform.tfstate"
    dynamodb_table = "oliver-bilbie-tf-lock-table"
    encrypt        = true
  }
}
