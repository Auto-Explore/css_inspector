# css/css-anchor-position/position-try-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-001.html"
}
```

## style[0]

```css

.cb {
  position: relative;
  width: 190px;
  height: 70px;
  background: yellow;
  border-bottom: 1px solid black;
}
.spacer {
  width: 1px;
  height: 20px;
}
.anchor1 {
  anchor-name: --a1;
  margin-left: 45px;
  width: 100px;
  height: 30px;
  background: blue;
}
.target {
  position: absolute;
  position-try-fallbacks: --f1, --f2, --f3, --f4;
  width: 40px;
  height: 15px;
  margin: 5px;
  background: orange;
  /* Position to the right of the anchor. */
  left: anchor(--a1 right);
  top: anchor(--a1 top);
}
@position-try --f1 {
  inset: auto;
  /* Position to the left of the anchor. */
  right: anchor(--a1 left);
  top: anchor(--a1 top);
}
@position-try --f2 {
  inset: auto;
  /* Position to the bottom of the anchor. */
  left: anchor(--a1 left);
  top: anchor(--a1 bottom);
}
@position-try --f3 {
  inset: auto;
  /* Position to the top of the anchor. */
  left: anchor(--a1 left);
  bottom: anchor(--a1 top);
}
@position-try --f4 {
  inset: auto;
  /* Position to the left with the narrower width. */
  left: anchor(--a1 right);
  top: anchor(--a1 top);
  width: 35px;
  height: 40px;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
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
