# css/css-grid/layout-algorithm/auto-margins-ignored-during-track-sizing-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/auto-margins-ignored-during-track-sizing-001.html"
}
```

## style[0]

```css

body { overflow: hidden; }
.grid {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
}
.margin-center {
  margin-top: 10px;
  margin-left: auto;
  margin-right: auto;
}
.i1 { background: magenta;  }
.i2 { background: cyan; }
.i3 { background: yellow; }
.i4 { background: lime; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
