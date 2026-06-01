group "default" {
  targets = [ "kotlin", "rust", "python" ]
}

target "_common" {
  dockerfile = "../Dockerfile"
  args = {
    NIX_VERSION = "2.32.8"
  }
  platforms = [ "linux/amd64" ]
}

target "kotlin" {
  inherits = [ "_common" ]
  context = "./kotlin"
  tags = [ "rudof/kotlin_builder:latest" ]
}

target "rust" {
  inherits = [ "_common" ]
  context = "./rust"
  tags = [ "rudof/rust_builder:latest" ]
}

target "python" {
  inherits = [ "_common" ]
  context = "./python"
  tags = [ "rudof/python_builder:latest" ]
}