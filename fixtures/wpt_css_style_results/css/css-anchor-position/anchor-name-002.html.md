# css/css-anchor-position/anchor-name-002.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-name-002.html"
}
```

## style[0]

```css

.relpos {
  position: relative;
}
.abspos {
  position: absolute;
}
.anchor1 {
  anchor-name: --a1;
  width: 10px;
  height: 10px;
  background: orange;
}
.target {
  position: absolute;
  width: anchor-size(--a1 width);
  height: 10px;
  background: lime;
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
