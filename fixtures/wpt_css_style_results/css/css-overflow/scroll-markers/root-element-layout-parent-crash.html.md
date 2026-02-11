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
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “scroll-marker-group”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
