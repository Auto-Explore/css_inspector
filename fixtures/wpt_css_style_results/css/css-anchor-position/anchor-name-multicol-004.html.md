# css/css-anchor-position/anchor-name-multicol-004.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-name-multicol-004.html"
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
  columns: 6;
  column-fill: auto;
  column-gap: 10px;
  width: 530px;
  height: 100px;
}
.spacer {
  height: 10px;
}
.anchor {
  anchor-name: --a1;
  width: 10px;
}
.target {
  position: absolute;
  left: anchor(--a1 left);
  top: anchor(--a1 top);
  width: anchor-size(--a1 width);
  height: anchor-size(--a1 height);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
