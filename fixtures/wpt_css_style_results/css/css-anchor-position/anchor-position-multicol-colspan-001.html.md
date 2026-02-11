# css/css-anchor-position/anchor-position-multicol-colspan-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-multicol-colspan-001.html"
}
```

## style[0]

```css

.relpos {
  position: relative;
}
.columns {
  column-count: 2;
  column-fill: auto;
  column-gap: 10px;
  column-width: 100px;
  width: 210px;
  height: 100px;
}
.colspan {
  column-span: all;
}
.spacer {
  height: 10px;
  background: pink;
}
.anchor1 {
  anchor-name: --a1;
  margin-left: 10px;
  width: 40px;
  height: 20px;
  background: orange;
}
.target {
  position: absolute;
  position-anchor: --a1;
  left: anchor(left);
  top: anchor(top);
  width: anchor-size(width);
  height: anchor-size(height);
  background: lime;
  opacity: .3;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
