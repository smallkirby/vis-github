# vis-github

![vis-github](img/img4.png)
![vis-github](img/img5.png)
![vis-github](img/img6.png)
![vis-github](img/img7.png)

# About

`vis-github` is a simple Github statistics CLI visualizer.

# Build

```build.sh
cargo build
./target/debug/vis-github --token "$GITHUB_API_TOKEN" fetch --owner <your name>
./target/debug/vis-github vis --owner <your name>
```

# Note

- `vis-github` downloads repository and related information, which would consume your API rate limit. Be careful not to repeat data fetching too much if you have many repos.
