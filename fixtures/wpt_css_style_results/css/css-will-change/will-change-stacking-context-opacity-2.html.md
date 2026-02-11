# css/css-will-change/will-change-stacking-context-opacity-2.html

```json
{
  "format_version": 3,
  "file": "css/css-will-change/will-change-stacking-context-opacity-2.html"
}
```

## style[0]

```css

  html, body { margin: 0; }
  .indicator {
    position: absolute;
    width: 100px;
    height: 100px;
    background: green;
    z-index: 1;
  }
  .will-change-opacity {
    will-change: opacity;
    width: 100px;
    height: 100px;
  }
  .top {
    position: absolute;
    width: 100px;
    height: 100px;
    background: red;
    z-index: 3;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
