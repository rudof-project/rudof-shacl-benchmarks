group "default" {
  targets = [ "rudof_v1", "rudof_v2" ]
}

target "_common" {
  platforms = [ "linux/amd64" ]
}

target "_common_rust" {
  inherits = [ "_common" ]
  dockerfile = "../Dockerfile_rust"
}

target "rudof_v1" {
  inherits = [ "_common_rust" ]
  args = {
    BINARY_NAME = "rudof_v1"
  }
  context = "./rudof_v1"
  tags = [ "rudof/rudof_v1:latest" ]
}

target "rudof_v2" {
  inherits = [ "_common_rust" ]
  args = {
    BINARY_NAME = "rudof_v2"
  }
  context = "./rudof_v2"
  tags = [ "rudof/rudof_v2:latest" ]
}
