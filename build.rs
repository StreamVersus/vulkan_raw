fn main() {
    let mut build = cc::Build::new();
    build.include("VulkanMemoryAllocator/include");

    if let Ok(lib) = pkg_config::probe_library("vulkan") {
        for path in &lib.include_paths {
            build.include(path);
        }
    } else if let Ok(sdk) = std::env::var("VULKAN_SDK") {
        build.include(format!("{}/include", sdk));
    } else {
        panic!(
            "Vulkan headers not found.\n\
                Install libvulkan-dev (Linux), the LunarG Vulkan SDK, \
                or set the VULKAN_SDK environment variable."
        );
    }

    build.file("VMAWrapper/vma.cpp");

    if cfg!(feature = "VK_VERSION_1_3") {
        build.define("VMA_VULKAN_VERSION", "1003000");
    } else {
        build.define("VMA_VULKAN_VERSION", "1002000");
    }

    let target = std::env::var("TARGET").unwrap();
    if target.contains("windows") {
        if target.contains("gnu") {
            build.flag("-std=c++14").cpp_link_stdlib("stdc++");
        } else {
            build.flag("/std:c++14").flag("/W0"); // MSVC: suppress all warnings
        }
    } else if target.contains("darwin") {
        build.flag("-std=c++14").flag("-w").cpp_link_stdlib("c++").cpp_set_stdlib("c++");
    } else if target.contains("android") {
        build.flag("-std=c++14").flag("-w").cpp_link_stdlib("c++");
    } else if target.contains("linux") {
        build.flag("-std=c++14").flag("-w").cpp_link_stdlib("stdc++");
    }
    build.cpp(true);
    build.compile("vma");
}
