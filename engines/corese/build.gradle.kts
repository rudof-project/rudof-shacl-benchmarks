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
    implementation(libs.corese.core)
}

kotlin {
    jvmToolchain(21)
}

configurations.all {
    resolutionStrategy.eachDependency {
        if (requested.group == "com.ibm.icu" && requested.name == "icu4j") {
            useVersion("74.2")
        }
    }
}

tasks.shadowJar {
    archiveClassifier.set("")

    manifest {
        attributes["Main-Class"] = "es.weso.rudof.MainKt"
    }
}