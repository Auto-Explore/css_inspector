# css/css-grid/grid-items/grid-items-percentage-margins-vertical-rl-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-items-percentage-margins-vertical-rl-001.html"
}
```

## style[0]

```css

.grid {
  font: 10px/1 Ahem;
  grid-template-columns: 100px;
  height: 500px;
  justify-items: start;
  position: relative;
  writing-mode: vertical-rl;
}

.grid > div:nth-child(1) { background: cyan; }
.grid > div:nth-child(2) {
  background: magenta;
  width: 10px;
  height: 100%;
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
