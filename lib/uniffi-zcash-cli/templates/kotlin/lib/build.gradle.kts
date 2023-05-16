plugins {
    id("org.jetbrains.kotlin.jvm") version "1.8.10"

    `java-library`
    // Added the maven-publish plugin
    `maven-publish`
}

repositories {
   // Use Maven Central for resolving dependencies.
   mavenCentral()   
   // Use local repository for testing
   mavenLocal()
}

dependencies {
  // Add the JNA. No need to manually include the jar: https://github.com/java-native-access/jna
  implementation("net.java.dev.jna:jna:5.8.0")
}

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(17))
    }
}

// Inlude the .so files in the jar. Our shared library is there.
val libsDir = File("libs")
tasks.withType<Jar> {
    from(libsDir) { include("**/*.so") }
}

publishing {
   publications {
	create<MavenPublication>("mavenJava") {
        from(components["java"])
        pom {
		// Artifact coordinates and info. To be setup for production.
		groupId = "uniffi"
        artifactId = "zcash"
		version = "{{version}}"
                description.set("The librustzcash Kotlin FFI binding")
                url.set("https://github.com/eigerco/uniffi-zcash-lib")
                licenses {
                    license {
                        name.set("The MIT License")
                        url.set("https://github.com/eigerco/uniffi-zcash-lib/blob/main/LICENSE")
                    }
                }
           }
	}
   }
   repositories {
     // Below auth config should be adjusted/used for production publishing.
     maven {
        url = uri("https://example.com/repository/maven")
        credentials {
            username = "token" // Use "token" as the username for API token authentication
            password = System.getenv("MAVEN_REPO_API_TOKEN")
        }
     }
  }
}