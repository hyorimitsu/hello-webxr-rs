hello-webxr-rs
---

This is a sample of WebXR.

## Description

This is a sample of WebXR in WebAssembly using Rust.

## Directory Structure

```
.
├── app-web              # => web sources
│    ├── shader            # => vertex shader & fragment shader
│    ├── src               # => Rust sources
│    └── (some omitted)
├── tools                # => local development tools
└── (some omitted)
```

## Provision

1. Build the Rust application.

    ```shell
    make build
    ```

2. Install the dependencies by [package.json](https://github.com/hyorimitsu/hello-webxr-rs/blob/main/app-web/package.json).

    ```shell
    make deps
    ```

3. Build the web application.

    ```shell
    make yarn-build
    ```

## Usage

1. Run the application.

    ```shell
    make run
    ```

2. Access the following URL.

    http://localhost:8080/
    ![img](https://user-images.githubusercontent.com/52403055/176657171-4f7f7ef1-18dc-425c-ae06-5752caba0c57.png)

3. down the application.

    ```shell
    make down
    ```
