# wasm

你可以只用 wasm-bindgen 来构建一个 wgpu 应用程序，但我在这样做的时候遇到了一些问题。首先，你需要在电脑上安装 wasm-bindgen，并将其作为一个依赖项。作为依赖关系的版本需要与你安装的版本完全一致，否则构建将会失败。

[wasm-pack](https://rustwasm.github.io/docs/wasm-pack/) 可以为你安装正确的 wasm-bindgen 版本，而且它还支持为不同类型的 web 目标进行构建：浏览器、NodeJS 和 webpack 等打包工具。

使用 wasm-pack 前需要先[安装](https://rustwasm.github.io/wasm-pack/installer/)。

完成安装后，就可以用它来构建我们的项目了。当你的项目是一个独立的包（crate）时，可以直接使用 wasm-pack build。如果是工作区（workspace），就必须指定你要构建的包。想象一下包是一个名为 game 的目录，你就会使用：

```shell
wasm-pack build game
```

一旦 wasm-pack 完成构建，在你的包目录下就会有一个 pkg 目录，运行 WASM 代码所需的所有 javascript 代码都在这里。然后在 javascript 中导入 WASM 模块：

```shell
const init = await import('./pkg/game.js');
init().then(() => console.log("WASM Loaded"));
```

html 示例：

```html
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Pong with WASM</title>
</head>

<body>
  <script type="module">
      import init from "./pkg/pong.js";
      init().then(() => {
          console.log("WASM Loaded");
      });
  </script>
  <style>
      canvas {
          background-color: black;
      }
  </style>
</body>

</html>

```
