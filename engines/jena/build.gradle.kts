plugins {
    alias(libs.plugins.kotlin.jvm)
    alias(libs.plugins.johnrengelman.shadow)
    alias(libs.plugins.kotlin.serialization)
}

group = "es.weso.rudof"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation(libs.jena.shacl)
    implementation(libs.jena.geosparql)
    implementation(libs.kotlinx.serialization.json)
}

kotlin {
    jvmToolchain(21)
}

tasks.shadowJar {
    archiveClassifier.set("")
    mergeServiceFiles()

    manifest {
        attributes["Main-Class"] = "es.weso.rudof.MainKt"
    }
}
