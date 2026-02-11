# css/css-anchor-position/anchor-position-multicol-nested-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-multicol-nested-001.html"
}
```

## style[0]

```css

.cb {
  position: relative;
}
body > * .cb {
  border-style: solid;
  border-color: transparent;
  border-width: 3px 2px 4px 1px;
  padding: 3px 2px 4px 1px;
}
.columns {
  columns: 4;
  column-fill: auto;
  column-gap: 10px;
  width: 630px;
  height: 100px;
}
.columns .columns {
  columns: 2;
  width: 130px;
  height: 200px;
}
.anchor1 {
  anchor-name: --a1;
  margin-left: 10px;
  width: 20px;
  height: 150px;
  background: red;
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
