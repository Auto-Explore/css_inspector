# css/css-grid/grid-items/grid-items-percentage-paddings-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-items-percentage-paddings-001.html"
}
```

## style[0]

```css

.grid {
  font: 10px/1 Ahem;
  grid-template-columns: 100px;
  width: 500px;
  justify-items: start;
  position: relative;
}

.grid > div:nth-child(1) { background: cyan; }
.grid > div:nth-child(2) {
  background: magenta;
  width: 100%;
  height: 10px;
}

.paddingLeft50Percent { padding-left: 50%; }
.paddingRight50Percent { padding-right: 50%; }
.paddingTop50Percent { padding-top: 50%; }
.paddingBottom50Percent { padding-bottom: 50%; }
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
