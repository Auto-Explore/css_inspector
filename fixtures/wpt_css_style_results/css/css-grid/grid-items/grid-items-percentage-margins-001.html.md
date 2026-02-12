# css/css-grid/grid-items/grid-items-percentage-margins-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-items-percentage-margins-001.html"
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

.marginLeft50Percent { margin-left: 50%; }
.marginRight50Percent { margin-right: 50%; }
.marginTop50Percent { margin-top: 50%; }
.marginBottom50Percent { margin-bottom: 50%; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
