default: dev images
	docker compose up

# building docker images --------------------
dev:
	./scripts/build_dev.sh

release:
	./scripts/build_release.sh

#  images -----------------------------------
images: dev
	./scripts/docker_images.sh

images-release: release
	echo "Implement me!"
