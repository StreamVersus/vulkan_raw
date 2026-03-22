use std::env;

fn main() {
    #[cfg(feature = "VulkanMemoryAllocator")]
    {
        let mut build = cc::Build::new();
        build.include("VulkanMemoryAllocator/include");
        build.include("Vulkan-Headers/include");
        build.file("VMAWrapper/vma.cpp");

        if cfg!(feature = "VK_VERSION_1_3") {
            build.define("VMA_VULKAN_VERSION", "1003000");
        } else {
            build.define("VMA_VULKAN_VERSION", "1002000");
        }

        let target = env::var("TARGET").unwrap();
        if target.contains("windows") {
            let xwin_dir = env::var("XWIN_DIR")
                .expect("XWIN_DIR must be set when targeting Windows");

            build.flag(format!("-imsvc{xwin_dir}/crt/include"));
            build.flag(format!("-imsvc{xwin_dir}/sdk/include/ucrt"));
            build.flag(format!("-imsvc{xwin_dir}/sdk/include/um"));
            build.flag(format!("-imsvc{xwin_dir}/sdk/include/shared"));
            build.flag("/std:c++17");
        } else {
            build.flag("-std=c++17");

            if target.contains("darwin") {
                build.cpp_link_stdlib("c++");
                build.cpp_set_stdlib("c++");
            } else if target.contains("android") {
                build.cpp_link_stdlib("c++");
            } else {
                // linux, gnu, etc.
                build.cpp_link_stdlib("stdc++");
            }
        }

        build.warnings(false);
        if target.contains("darwin") {
            build.cpp_link_stdlib("c++").cpp_set_stdlib("c++");
        } else if target.contains("android") {
            build.cpp_link_stdlib("c++");
        } else if target.contains("linux") || (target.contains("windows") && target.contains("gnu")) {
            build.cpp_link_stdlib("stdc++");
        }
        build.cpp(true);
        build.compile("vma");
    }
}
