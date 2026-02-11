# css/css-position/hypothetical-dynamic-change-001.html

```json
{
  "format_version": 3,
  "file": "css/css-position/hypothetical-dynamic-change-001.html"
}
```

## style[0]

```css

  .ancestor, .child {
    position: fixed;
    width: 100px;
    height: 100px;
    background-color: green;
    /* NOTE: child remains auto-positioned */
  }
  .ancestor {
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
