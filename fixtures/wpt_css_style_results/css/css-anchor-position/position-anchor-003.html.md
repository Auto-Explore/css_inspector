# css/css-anchor-position/position-anchor-003.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-anchor-003.html"
}
```

## style[0]

```css

#target {
  position: fixed;
  width: 100px;
  height: 100px;
  background: lime;
  top: anchor(top);
  left: anchor(right);
  position-anchor: --a;
}

#target.after {
  position-anchor: --b;
}

#anchor1, #anchor2 {
  width: 100px;
  height: 100px;
  background: orange;
}

#anchor1 {
  anchor-name: --a;
}

#anchor2 {
  margin-left: 100px;
  anchor-name: --b;
}

body {
  margin: 0;
}
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
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
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
