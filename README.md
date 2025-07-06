### rust entrance
[Youtube](https://www.youtube.com/watch?v=DEWoizX96k8&ab_channel=TonyJohnson)

```shell
$ rustup upgrade
$ rustup target add wasm32-unknown-unknown
$ cargo new yew-tailwind
$ cd yew-tailwind
$ cargo install trunk
$ cargo install wasm-bindgen-cli
$ trunk serve
```

### tailwind
- tailwindcss v4.1.11 20250706
```shell
$ npm install tailwindcss @tailwindcss/cli
$ npx tailwindcss -o styles/tailwindv4.1.11.css
```
- 初始化
```shell
$ npm install -D tailwindcss
```
`styles/input.css`
```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```
- 生成tailwind css文件
```
$ npx tailwindcss -i ./styles/input.css -o ./styles/output.css
```

### tailwindcss v4

doc：https://tailwindcss.com/docs/v4-beta

```shell
$ npm install tailwindcss@next @tailwindcss/cli@next
```
`styles/input.css`
```css
@import "tailwindcss";
```

```shell
$ npx @tailwindcss/cli -i styles/input.css -o styles/tailwindcss_v4.css
```

### 以trunk语法引入css
`index.html`
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <link data-trunk rel="css" href="/styles/output.css" />
    <title>Yew App</title>
  </head>
  <body>

  </body>
</html>
```

### .rs引入css
`main.rs`
```rust
use yew::prelude::*;

#[function_component(HelloWorld)]
fn hello_world() -> Html {
    html!(
        <h1 class="text-base/7 font-semibold text-zinc-950 sm:text-sm/6 dark:text-white">
            {"Hello World"}
        </h1>
    )
}

fn main() {
    yew::start_app::<HelloWorld>();
}
```

### 基于tailwindcss的UI组件库：https://preline.co/docs/index.html