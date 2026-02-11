# css/css-position/hypothetical-dynamic-change-002.html

```json
{
  "format_version": 3,
  "file": "css/css-position/hypothetical-dynamic-change-002.html"
}
```

## style[0]

```css

  .ancestor, .child {
    width: 100px;
    height: 100px;
    background-color: green;
  }
  .child {
    position: fixed;
    /* NOTE: child remains auto-positioned */
  }
  .ancestor {
    position: absolute;
    left: 0;
    top: 0;
    background-color: red;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
