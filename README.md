### rust entrance
[Youtube](https://www.youtube.com/watch?v=DEWoizX96k8&ab_channel=TonyJohnson)

```shell
$ cargo new yew-tailwind
$ cd yew-tailwind
$ trunk serve
```

### tailwind
- 初始化
```shell
$ npm install -D tailwindcss
$ npx tailwindcss init
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
        <h1>{"Hello World"}</h1>
    )
}

fn main() {
    yew::start_app::<HelloWorld>();
}
```