# css/css-view-transitions/view-transition-name-is-grouping.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/view-transition-name-is-grouping.html"
}
```

## style[0]

```css

.parent {
  top: 0;
  width: 100px;
  height: 100px;
  position: absolute;
  background: red;
  transform-style: preserve-3d;
  view-transition-name: target;
}

.child {
  background: green;
  width: 100px;
  height: 100px;
  top: 0;
  left: 0;
  position: absolute;
  transform: translateZ(-500px);
}

body {
  perspective: 1000px;
  height: 500px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
