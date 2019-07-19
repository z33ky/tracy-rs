fn main() {
    let mut cc_build = cc::Build::new();
    cc_build.use_plt(false)
            .flag_if_supported("-flto")
            .flag_if_supported("/LTCG")
            .cpp(true)
            .flag_if_supported("-Wno-implicit-fallthrough")
            .flag_if_supported("-Wno-unused-function")
            .flag_if_supported("-Wno-unused-parameter")
            .include("./tracy");

    if cfg!(feature = "dll") {
        cc_build.file("./tracy/TracyClientDLL.cpp");
    } else {
        cc_build.file("./tracy/TracyClient.cpp");
    }

    if cfg!(feature = "enable") {
        cc_build.define("TRACY_ENABLE", None);
    }

    if cfg!(feature = "no_exit") {
        cc_build.define("TRACY_NO_EXIT", None);
    }

    if cfg!(feature = "on_demand") {
        cc_build.define("TRACY_ON_DEMAND", None);
    }

    cc_build.compile("libtracy.a");
}
