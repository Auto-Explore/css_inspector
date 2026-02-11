# css/css-viewport/zoom/container-queries.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/container-queries.html"
}
```

## style[0]

```css

  .container {
    container-type: inline-size;
    width: 100px;
    height: 100px;
  }
  .child {
    background-color: green;
    height: 50px;
    width: 50px;
    @container (width > 120px) {
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
