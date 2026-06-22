group "default" {
  targets = [ "rust", "kotlin", "python" ]
}

group "rust" {
  targets = [ "rudof_v1", "rudof_v2", "rudof_qlever" ]
}

group "kotlin" {
  targets = [ "corese", "rdf4j", "rdfunit", "topbraid", "jena" ]
}

group "python" {
  targets = [ "pyshacl", "maplib", "pyshacl_n" ]
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

target "rudof_qlever" {
  inherits = [ "_common_rust" ]
  dockerfile = "Dockerfile"
  args = {
    BINARY_NAME = "rudof_qlever"
  }
  context = "./rudof_qlever"
  tags = [ "rudof/rudof_qlever:latest" ]
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
  dockerfile = "Dockerfile"
  args = {
    BINARY_NAME = "pyshacl"
  }
  context = "./pyshacl"
  tags = [ "rudof/pyshacl:latest" ]
}

target "pyshacl_n" {
  inherits = [ "_common_python" ]
  context = "./pyshacl"
  tags = [ "rudof/pyshacl_n:latest" ]
}

target "maplib" {
  inherits = [ "_common_python" ]
  context = "./maplib"
  tags = [ "rudof/maplib:latest" ]
}
