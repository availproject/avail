podman run --rm --network host -p 9090:9090  -v ./prom_config.yml:/etc/prometheus/prometheus.yml:Z prom/prometheus
