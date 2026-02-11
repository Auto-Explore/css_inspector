# css/css-viewport/zoom/container-queries-height.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/container-queries-height.html"
}
```

## style[0]

```css

  .container {
    container-type: size;
    width: 100px;
    height: 100px;
  }
  .child {
    background-color: green;
    height: 50px;
    width: 50px;
    @container (height > 120px) {
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
