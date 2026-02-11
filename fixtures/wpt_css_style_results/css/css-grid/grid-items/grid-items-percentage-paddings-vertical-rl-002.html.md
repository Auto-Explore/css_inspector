# css/css-grid/grid-items/grid-items-percentage-paddings-vertical-rl-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-items-percentage-paddings-vertical-rl-002.html"
}
```

## style[0]

```css

.grid {
  font: 10px/1 Ahem;
  grid-template-columns: minmax(auto, 100px);
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
