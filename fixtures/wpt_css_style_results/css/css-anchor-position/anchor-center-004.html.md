# css/css-anchor-position/anchor-center-004.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-center-004.html"
}
```

## style[0]

```css

.container {
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
  left: 30px;
  top: 20px;
  background: lime;
}

.target {
  position-anchor: --anchor;
  width: 24px;
  height: 24px;
  position: fixed;
  background: cyan;
}

.justify {
  justify-self: anchor-center;
  top: anchor(bottom);
  /* Should be overridden to 0. */
  margin-inline: auto;
}

.align {
  align-self: anchor-center;
  right: anchor(left);
  /* Should be overridden to 0. */
  margin-block: auto;
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
