# css/css-anchor-position/anchor-center-vrl-vrl.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-center-vrl-vrl.html"
}
```

## style[0]

```css

.container {
  writing-mode: vertical-rl;
  width: 100px;
  height: 100px;
  border: solid 3px;
  position: relative;
  margin: 50px;
}

.anchor {
  anchor-name: --anchor;
  position: relative;
  width: 50px;
  height: 50px;
  right: 40px;
  top: 40px;
  background: lime;
}

.target {
  position-anchor: --anchor;
  position: absolute;
  background: cyan;
  justify-self: anchor-center;
  right: anchor(left);
  width: 20px;
}
/* used to test the available-size given to the element */
.target::after {
  color: transparent;
  content: 'a a a a a a a a a a a a a a a a a a';
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
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
