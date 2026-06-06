group "default" {
  targets = [ "rudof_v1", "rudof_v2", "corese", "rdf4j", "rdfunit", "topbraid", "jena" ]
}

target "_common" {
  platforms = [ "linux/amd64" ]
  args = {
    ROOTFS = "busybox:1.38-musl"
  }
}

target "_common_rust" {
  inherits = [ "_common" ]
  dockerfile = "../Dockerfile_rust"
}

target "_common_kt" {
  inherits = [ "_common" ]
  dockerfile = "../Dockerfile_kt"
}

target "_common_python" {
  inherits = [ "_common" ]
  dockerfile = "../Dockerfile_python"
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

target "corese" {
  inherits = [ "_common_kt" ]
  args = {
    JAR_NAME = "corese"
  }
  context = "./corese"
  tags = [ "rudof/corese:latest" ]
}

target "rdf4j" {
  inherits = [ "_common_kt" ]
  args = {
    JAR_NAME = "rdf4j"
  }
  context = "./rdf4j"
  tags = [ "rudof/rdf4j:latest" ]
}

target "rdfunit" {
  inherits = [ "_common_kt" ]
  args = {
    JAR_NAME = "rdfunit"
  }
  context = "./rdfunit"
  tags = [ "rudof/rdfunit:latest" ]
}

target "topbraid" {
  inherits = [ "_common_kt" ]
  args = {
    JAR_NAME = "topbraid"
  }
  context = "./topbraid"
  tags = [ "rudof/topbraid:latest" ]
}

target "jena" {
  inherits = [ "_common_kt" ]
  args = {
    JAR_NAME = "jena"
  }
  context = "./jena"
  tags = [ "rudof/jena:latest" ]
}

target "pyshacl" {
  inherits = [ "_common_python" ]
  args = {
    BINARY_NAME = "pyshacl"
  }
  context = "./pyshacl"
  tags = [ "rudof/pyshacl:latest" ]
}
