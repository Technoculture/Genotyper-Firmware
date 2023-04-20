# Config Files
1. Parser Libary and Binary
2. Docker for a bunch of zenoh "modules"
3. Docker for main node
4. Docker Compose to run it all

```txt
Kubernetes Manifests <------- ?
|  Container Registry <------ Containers
|  |  Config & Library <----- Cloud Git-like File Storage
|  |  |             ^-------- Cloud JSON Db
ˇ  ˇ  ˇ  
Deployements ----|
                 |----------> Fluentbit telemetry
                 |----------> Sentry Error Logs
                 |----------> Events Log Storage
```

