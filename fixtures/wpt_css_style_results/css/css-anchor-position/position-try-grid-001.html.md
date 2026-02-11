# css/css-anchor-position/position-try-grid-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-grid-001.html"
}
```

## style[0]

```css

.cb {
  position: relative;
}
.grid {
  display: grid;
  grid-template-columns: repeat(4, 100px);
  grid-template-rows: 50px 100px 50px 50px;
}
.item {
  background: lightgray;
}
.spacer {
  background: yellow;
}
.anchor1 {
  anchor-name: --a1;
  background: orange;
  margin-left: 15px;
  width: 20px;
  height: 30px;
}
.target {
  grid-column: 2 / 4;
  grid-row: 2 / 4;
  position: absolute;
  position-try-fallbacks: --f1, --f2, --f3;
  width: 100px;
  height: 100px;
  background: lime;
  opacity: .2;
  /* Position to the left of the anchor. */
  position-anchor: --a1;
  right: anchor(left);
  top: anchor(top);
}
@position-try --f1 {
  inset: auto;
  left: anchor(right);
  top: anchor(top);
  width: 250px;
}
@position-try --f2 {
  /* Position to the right of the anchor. This entry should succeed. */
  inset: auto;
  left: anchor(right);
  top: anchor(top);
}
@position-try --f3 {
  /* Zero-sized, the last entry wins if none succeeded. */
  inset: auto;
  left: 0;
  top: 0;
  width: 0;
  height: 0;
}
```

```json
{
  "errors": 8,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
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
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
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
