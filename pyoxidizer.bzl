def make_exe():
    dist = default_python_distribution()
    policy = dist.make_python_packaging_policy()
    policy.extension_module_filter = "no-copyleft"
    # https://github.com/indygreg/PyOxidizer/issues/438
    policy.resources_location = "filesystem-relative:lib"
    policy.resources_location_fallback = None

    python_config = dist.make_python_interpreter_config()
    python_config.run_command = "import querent"
    exe = dist.to_python_executable(
        name="pyquerent",
        packaging_policy=policy,
        config=python_config,
    )

    exe.add_python_resources(exe.pip_install([
        "querent==3.0.8",
        "torch@https://download.pytorch.org/whl/cpu/torch-2.0.1%2Bcpu-cp310-cp310-linux_x86_64.whl"
    ]))

    return exe

def make_embedded_resources(exe):
    return exe.to_embedded_resources()

def make_install(exe):
    # Create an object that represents our installed application file layout.
    files = FileManifest()

    # Add the generated executable to our install layout in the root directory.
    files.add_python_resource(".", exe)

    return files

# Tell PyOxidizer about the build targets defined above.
register_target("exe", make_exe)
register_target("resources", make_embedded_resources, depends=["exe"], default_build_script=True)
register_target("install", make_install, depends=["exe"], default=True)

# Resolve whatever targets the invoker of this configuration file is requesting
# be resolved.
resolve_targets()
