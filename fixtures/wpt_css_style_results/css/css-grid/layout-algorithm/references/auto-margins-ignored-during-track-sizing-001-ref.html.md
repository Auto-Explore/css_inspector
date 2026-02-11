# css/css-grid/layout-algorithm/references/auto-margins-ignored-during-track-sizing-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/references/auto-margins-ignored-during-track-sizing-001-ref.html"
}
```

## style[0]

```css

body { overflow: hidden; }
.grid {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
}
.margin { margin-top: 10px; }
.center { justify-self: center; }
.i1 { background: magenta;  }
.i2 { background: cyan; }
.i3 { background: yellow; }
.i4 { background: lime; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
