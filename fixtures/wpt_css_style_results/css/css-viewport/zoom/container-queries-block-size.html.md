# css/css-viewport/zoom/container-queries-block-size.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/container-queries-block-size.html"
}
```

## style[0]

```css

  .container {
    container-type: size;
    width: 100px;
    height: 200px;
    writing-mode: vertical-lr;
  }
  .child {
    background-color: green;
    height: 50px;
    width: 50px;
    @container (block-size > 120px) {
      background-color: red;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
