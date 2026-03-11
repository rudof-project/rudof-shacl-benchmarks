plugins {
    alias(libs.plugins.kotlin.jvm)
    alias(libs.plugins.johnrengelman.shadow)
}

group = "es.weso.rudof"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation(libs.jena.core)
    implementation(libs.jena.arq)
    implementation(libs.rdfunit.core)
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