# css/css-will-change/will-change-stacking-context-backdrop-filter-1.html

```json
{
  "format_version": 3,
  "file": "css/css-will-change/will-change-stacking-context-backdrop-filter-1.html"
}
```

## style[0]

```css

  html, body { margin: 0; }
  .indicator {
    position: absolute;
    background-color: green;
    z-index: 1;
  }
  .willchange {
    will-change: backdrop-filter;
    z-index: 0;
  }
  .child {
    position: relative;
    background-color: red;
    z-index: 2;
  }
  .box {
    width: 100px;
    height: 100px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
