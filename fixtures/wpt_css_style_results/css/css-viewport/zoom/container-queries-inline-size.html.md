# css/css-viewport/zoom/container-queries-inline-size.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/container-queries-inline-size.html"
}
```

## style[0]

```css

  .container {
    container-type: inline-size;
    width: 200px;
    height: 100px;
    writing-mode: vertical-lr;
  }
  .child {
    background-color: green;
    height: 50px;
    width: 50px;
    @container (inline-size > 120px) {
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
