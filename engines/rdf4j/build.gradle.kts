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
    implementation(platform(libs.rdf4j.bom))
    implementation(libs.rdf4j.shacl)
    implementation(libs.rdf4j.rio.turtle)
    implementation(libs.rdf4j.storage)
    implementation(libs.rdf4j.queryalgebra.geosparql)
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
