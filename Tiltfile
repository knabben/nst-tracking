#k8s_yaml('deploy.yaml')

docker_build("frontend", "./front/",
  live_update=[
    fall_back_on("front/package.json"),
    sync("./front/src", "/app/src")
  ])
k8s_yaml("front/deploy.yaml")
k8s_resource("frontend", port_forwards=3000)
