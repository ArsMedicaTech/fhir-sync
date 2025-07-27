include .env

docker-create:
	aws ecr create-repository --repository-name $(FHIR_SYNC_IMAGE) --region us-east-1 || true


auth:
	aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin $(DOCKER_REGISTRY)


FHIR_SYNC_BUILD_ARGS=

fhir-sync-docker:
	docker build -t $(DOCKER_REGISTRY)/$(FHIR_SYNC_IMAGE):$(FHIR_SYNC_VERSION) $(FHIR_SYNC_BUILD_ARGS) -f ./Dockerfile .
	docker push $(DOCKER_REGISTRY)/$(FHIR_SYNC_IMAGE):$(FHIR_SYNC_VERSION)
	kubectl rollout restart deployment $(FHIR_SYNC_DEPLOYMENT) --namespace=$(NAMESPACE)



k8s-auth:
	kubectl create secret docker-registry ecr-secret --docker-server=$(DOCKER_REGISTRY) --docker-username=AWS --docker-password=$(DOCKER_PASSWORD) --namespace=$(NAMESPACE)

k8s-deploy:
	kubectl create namespace $(NAMESPACE) || true
	helm upgrade --install $(NAMESPACE) ../k8s --namespace $(NAMESPACE) -f ../k8s/values.yaml

