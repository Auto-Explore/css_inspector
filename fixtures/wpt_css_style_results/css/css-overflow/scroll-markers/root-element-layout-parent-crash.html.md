# css/css-overflow/scroll-markers/root-element-layout-parent-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/root-element-layout-parent-crash.html"
}
```

## style[0]

```css

  html {
    overflow: auto;
    scroll-marker-group: before;
  }
  html::scroll-marker-group {
    width: 100px;
    height: 100px;
  }
  html::scroll-button(left) {
    content: "";
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
