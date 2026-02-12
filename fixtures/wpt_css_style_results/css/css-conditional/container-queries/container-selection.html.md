# css/css-conditional/container-queries/container-selection.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-selection.html"
}
```

## style[0]

```css


  main { background-color: lightgray; }
  main > div { background-color: skyblue; }
  main > div > div { background-color: seagreen; }
  main > div > div > div { background-color: tomato; }

  main {
    width: 64px;
    height: 64px;
  }

  main div {
    width: 50%;
    height: 50%;
  }

  .inline { container-type: inline-size; }
  .size { container-type: size; }

  .a-inline { container: a / inline-size; }
  .a-size { container: a / size; }

  .b-size { container: inline- b / size; }
  .b-size { container: b / size; }

  .ab-size { container: a b / size; }

  .a { container-name: a; contain: strict; }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
