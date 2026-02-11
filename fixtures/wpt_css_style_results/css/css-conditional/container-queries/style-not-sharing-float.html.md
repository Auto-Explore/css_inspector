# css/css-conditional/container-queries/style-not-sharing-float.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/style-not-sharing-float.html"
}
```

## style[0]

```css

  .float {
    float: left;
    width: 25px;
    height: 25px;
  }
  .item {
    container-type: inline-size;
    height: 25px;
  }
  @container (width >= 50px) {
    .item div { color: lime; }
  }
  @container (width >= 150px) {
    .item div { color: green; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
