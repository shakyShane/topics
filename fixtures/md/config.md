# Config: global-vars

These are variables that are deemed 'global' in the sense that they may be useful
to any individual part of the project

```toml
images.client = { tag = "nbs-frontend", context = "./containers/www/client" } 
images.graph = { tag = "nbs-graph", context = "./containers/www/graph/api" } 
```