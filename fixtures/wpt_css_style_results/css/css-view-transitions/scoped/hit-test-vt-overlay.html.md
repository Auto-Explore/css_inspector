# css/css-view-transitions/scoped/hit-test-vt-overlay.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/hit-test-vt-overlay.html"
}
```

## style[0]

```css


body { margin: 0; }
#scope {
  position: relative; width: 100px; height: 110px;
  background: white; contain: strict; view-transition-name: none;
}
#part {
  position: absolute; top: 10px; left: 10px;
  width: 50px; height: 50px; background: #8cf;
  z-index: 1; view-transition-name: foo;
}
#higher-nonpart {
  position: absolute; top: 20px; left: 40px;
  z-index: 2; width: 50px; height: 50px; background: #f88;
}
#inflow-nonpart {
  background: #4f8; width: 50px; height: 50px;
  margin-left: 20px; margin-top: 50px;
}
::view-transition { background: rgba(0, 0, 0, 0.1); height: 40px; }
::view-transition-group(*) { animation-play-state: paused; }
::view-transition-new(*) { animation: unset; opacity: 1; }
::view-transition-old(*) { animation: unset; opacity: 0; }

```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
