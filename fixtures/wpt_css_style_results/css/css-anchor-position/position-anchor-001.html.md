# css/css-anchor-position/position-anchor-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-anchor-001.html"
}
```

## style[0]

```css

.anchor {
  width: 100px;
  height: 100px;
  background: orange;
  font: 20px/1 Ahem;
}

.target {
  position: fixed;
  background: lime;
  position-try-fallbacks: --pf;
  left: 999999px; /* force fallback */
  font: 20px/1 Ahem;
}

@position-try --pf {
  top: anchor(bottom, 0px);
  left: anchor(left, 0px);
  width: anchor-size(width, 0px);
  height: anchor-size(height, 0px);
}

body {
  margin: 0;
}

#anchor1 {
  anchor-name: --a1;
  margin-left: 100px;
}

#target1 {
  position-anchor: --a1;
}

#anchor2 {
  anchor-name: --a2;
  margin-left: 300px;
  margin-top: 100px;
}

#target2 {
  position-anchor: --a2;
}
```

```json
{
  "errors": 7,
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
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
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
