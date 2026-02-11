# css/css-view-transitions/scoped/crashtests/container-query.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/crashtests/container-query.html"
}
```

## style[0]

```css

  #container {
    width: 100px;
    container-type: inline-size;
  }
  @container (width > 200px) {
    #target { background-color: green }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
