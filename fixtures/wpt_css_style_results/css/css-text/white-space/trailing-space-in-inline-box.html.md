# css/css-text/white-space/trailing-space-in-inline-box.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/trailing-space-in-inline-box.html"
}
```

## style[0]

```css

.target {
  font-family: Consolas, 'Courier New', Courier, monospace;
  font-size: 20px;
  width: 5ch;
  white-space: pre-wrap;
  overflow: auto visible;
  border: 1px solid blue;
}
.not-culled span {
  background: orange;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “overflow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
