# css/css-anchor-position/anchor-name-multicol-002.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-name-multicol-002.html"
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
.columns {
  column-count: 2;
  column-fill: auto;
  column-gap: 10px;
  column-width: 100px;
  width: 210px;
  height: 100px;
}
.spacer {
  height: 10px;
  background: pink;
}
.anchor {
  anchor-name: --a1;
  margin-left: 10px;
  width: 40px;
  height: 60px;
  background: orange;
}
.target {
  position: absolute;
  background: lime;
  opacity: .5;
  position-anchor: --a1;
  left: anchor(left);
  top: anchor(top);
  width: anchor-size(width);
  height: anchor-size(height);
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
