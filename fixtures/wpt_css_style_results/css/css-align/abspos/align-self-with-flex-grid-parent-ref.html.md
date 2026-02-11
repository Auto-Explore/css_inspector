# css/css-align/abspos/align-self-with-flex-grid-parent-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-align/abspos/align-self-with-flex-grid-parent-ref.html"
}
```

## style[0]

```css

.container {
  position: relative;
  width: 300px;
  height: 300px;
  background: purple;
}

.abs {
  position: absolute;
  width: 100px;
  height: 100px;
  inset: 0;
  align-self: center;
  justify-self: center;
  background: pink;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
