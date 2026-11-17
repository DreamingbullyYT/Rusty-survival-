plugins { id("com.android.application") }

android {
    namespace = "com.rustysurvival.vulkanpoc"
    compileSdk = 35
    defaultConfig {
        applicationId = "com.rustysurvival.vulkanpoc"
        minSdk = 24
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"
    }
    sourceSets { getByName("main").jniLibs.srcDir("src/main/jniLibs") }
}
