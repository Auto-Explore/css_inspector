# css/css-position/position-absolute-crash-chrome-012.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-absolute-crash-chrome-012.html"
}
```

## style[0]

```css

  #container {
    width: 600px;
    position: relative;
  }
  #target {
    position: absolute;
    left: 0px;
    right: 33554000px;
    width: 10px;
    margin-left: 33554432px;
    margin-right: 33554432px;
    height: 20px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
