/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

extra["displayName"] = "S3D CODEGEN"
plugins { id("software.amazon.smithy").version("0.5.3") }

val smithyVersion: String by project
val runtimesOutputDir = buildDir.resolve("rust-runtime")

dependencies {
    implementation(project(":codegen-server"))
    implementation("software.amazon.smithy:smithy-aws-traits:$smithyVersion")
}

tasks["test"].finalizedBy("cargoCheck", "cargoClippy", "cargoTest", "cargoDocs")

tasks["assemble"].dependsOn("smithyBuildJar")

tasks["assemble"].finalizedBy(
    "copyAllRuntimes",
    "relocateAllRuntimes"
    // "fixManifests"
)

tasks.register("copyAllRuntimes") {
    doLast {
        copy {
            from("$rootDir/aws/rust-runtime") {
                CrateSet.AWS_SDK_RUNTIME.forEach { include("$it/**") }
            }
            from("$rootDir/rust-runtime") {
                CrateSet.SERVER_SMITHY_RUNTIME.forEach { include("$it/**") }
            }
            exclude("**/target", "**/Cargo.lock", "**/node_modules")
            into(runtimesOutputDir)
        }
        // TODO sts and s3 sdks are needed on runtime, and we just assume here that it is already built:
        // - ./gradlew -Paws.services=+sts,+sso,+s3 :aws:sdk:assemble
        copy {
            from("$rootDir/aws/sdk/build/smithyprojections/sdk/sts/rust-codegen")
            include("**")
            exclude("**/target", "**/Cargo.lock", "**/node_modules")
            into(runtimesOutputDir.resolve("sts")) // the `aws-config` crate depends on sts this way
        }
        copy {
            from("$rootDir/aws/sdk/build/smithyprojections/sdk/sso/rust-codegen")
            include("**")
            exclude("**/target", "**/Cargo.lock", "**/node_modules")
            into(runtimesOutputDir.resolve("sso")) // the `aws-config` crate depends on sts this way
        }
        copy {
            from("$rootDir/aws/sdk/build/smithyprojections/sdk/s3/rust-codegen")
            include("**")
            exclude("**/target", "**/Cargo.lock", "**/node_modules")
            into(runtimesOutputDir.resolve("aws-sdk-s3"))
        }
    }
}

tasks.register("relocateAllRuntimes") {
    dependsOn("copyAllRuntimes")
    doLast {
        val properties = PropertyRetriever(rootProject, project)

        // The aws/rust-runtime crates depend on local versions of the Smithy core runtime enabling local compilation. However,
        // those paths need to be replaced in the final build. We should probably fix this with some symlinking.
        fun rewritePathDependency(line: String): String {
            return line
                // some runtime crates are actually dependent on the generated bindings:
                .replace("../sdk/build/aws-sdk/sdk/", "")
                // others use relative dependencies::
                .replace("../../rust-runtime/", "")
        }

        // Patch the Cargo.toml files
        listOf("sts", "sso", "aws-sdk-s3").forEach { moduleName ->
            patchFile(runtimesOutputDir.resolve("$moduleName/Cargo.toml")) { line ->
                rewriteAwsSdkCrateVersion(properties, line.let(::rewritePathDependency))
            }
        }
        CrateSet.AWS_SDK_RUNTIME.forEach { moduleName ->
            patchFile(runtimesOutputDir.resolve("$moduleName/Cargo.toml")) { line ->
                rewriteAwsSdkCrateVersion(properties, line.let(::rewritePathDependency))
            }
        }
        CrateSet.SERVER_SMITHY_RUNTIME.forEach { moduleName ->
            patchFile(runtimesOutputDir.resolve("$moduleName/Cargo.toml")) { line ->
                rewriteSmithyRsCrateVersion(properties, line)
            }
        }
    }
}

tasks.register<Exec>("fixManifests") {
    dependsOn("assemble")
    dependsOn("copyAllRuntimes")
    dependsOn("relocateAllRuntimes")
    description = "Run the publisher tool's `fix-manifests` sub-command on the generated services"
    val publisherPath = rootProject.projectDir.resolve("tools/publisher")
    inputs.dir(publisherPath)
    outputs.dir(runtimesOutputDir)
    workingDir(publisherPath)
    environment(mapOf(
        "RUST_BACKTRACE" to "1",
        "RUST_LOG" to "debug"
    ))
    commandLine("cargo", "run", "--", "fix-manifests", "--location", runtimesOutputDir.absolutePath)
}

tasks.register<Exec>("cargoCheck") {
    workingDir("build/smithyprojections/s3d/")
    // disallow warnings
    environment("RUSTFLAGS", "-D warnings")
    commandLine("cargo", "check")
    dependsOn("assemble")
}

tasks.register<Exec>("cargoTest") {
    workingDir("build/smithyprojections/s3d/")
    // disallow warnings
    environment("RUSTFLAGS", "-D warnings")
    commandLine("cargo", "test")
    dependsOn("assemble")
}

tasks.register<Exec>("cargoDocs") {
    workingDir("build/smithyprojections/s3d/")
    // disallow warnings
    environment("RUSTFLAGS", "-D warnings")
    commandLine("cargo", "doc", "--no-deps")
    dependsOn("assemble")
}

tasks.register<Exec>("cargoClippy") {
    workingDir("build/smithyprojections/s3d/")
    // disallow warnings
    commandLine("cargo", "clippy", "--", "-D", "warnings")
    dependsOn("assemble")
}

