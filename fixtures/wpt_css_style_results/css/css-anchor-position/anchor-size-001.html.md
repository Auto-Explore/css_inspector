# css/css-anchor-position/anchor-size-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-size-001.html"
}
```

## style[0]

```css

.container {
  position: relative;
  height: 10px;
}
.anchor1 {
  anchor-name: --a1;
  width: 5px;
  height: 7px;
  background: orange;
}
.target {
  position: absolute;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
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
