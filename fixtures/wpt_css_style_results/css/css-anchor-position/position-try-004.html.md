# css/css-anchor-position/position-try-004.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-004.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

.cb {
  width: 300px;
  height: 150px;
  position: relative;
  background: lightgray;
}

.anchor {
  position: absolute;
  width: 100px;
  height: 100px;
  top: 25px;
  background: orange;
  anchor-name: --a;
}

.target {
  position: absolute;
  width: 100px;
  height: 100px;
  background: lime;
  position-try-fallbacks: --fallback;
  top: anchor(--a top);
  right: anchor(--a left);
  margin-top: 10px;
  margin-right: 10px;
}

@position-try --fallback {
  inset: auto;
  bottom: anchor(--a bottom);
  left: anchor(--a right);
  margin: 0;
  margin-bottom: 10px;
  margin-left: 10px;
}
```

```json
{
  "errors": 5,
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
