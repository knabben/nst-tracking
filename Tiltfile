#k8s_yaml('deploy.yaml')

# Transaction Processor
docker_build("tp", "./tp/",
  live_update=[
    fall_back_on("tp/Cargo.toml"),
    sync("./tp/src", "/app/src"),
    restart_container()
])
k8s_yaml("tp/deploy.yaml")

# Frontend
# docker_build("frontend", "./front/",
#   live_update=[
#     fall_back_on("front/package.json"),
#     sync("./front/src", "/app/src")
# ])
# k8s_yaml("front/deploy.yaml")
# k8s_resource("frontend", port_forwards=3000)
