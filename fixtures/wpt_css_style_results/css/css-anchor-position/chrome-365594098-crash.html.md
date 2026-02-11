# css/css-anchor-position/chrome-365594098-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/chrome-365594098-crash.html"
}
```

## style[0]

```css

#container {
  position: relative;
  width: 100px;
  height: 100px;
}

#anchor {
  position: absolute;
  anchor-name: --a;
  left: 0;
  top: 0;
  width: 10px;
  height: 10px;
  background: cornflowerblue;
}

#target {
  position: absolute;
  position-anchor: --a;
  left: 0;
  top: anchor(bottom);
  width: 10px;
  height: 80px;
  background: lime;
  position-try-fallbacks: --fallback;
}

@position-try --fallback {
  left: anchor(right);
  top: 0;
  width: 10px;
  height: 10%;
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
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
