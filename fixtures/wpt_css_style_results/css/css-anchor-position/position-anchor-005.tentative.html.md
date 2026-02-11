# css/css-anchor-position/position-anchor-005.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-anchor-005.tentative.html"
}
```

## style[0]

```css

body {
  margin: 0;
}
.wrapper {
  border: 10px solid grey;
  border-left-width: 7px;
  border-top-width: 9px;
  position: relative;
  width: 200px;
  height: 400px;
  padding: 11px 12px 8px 6px;
}
.a {
  background: green;
  width: 50px;
  height: 100px;
  margin: 30px 28px 32px 52px;
  border: 10px solid yellow;
  border-left-width: 8px;
  border-right-width: 7px;
  border-top-width: 6px;
  padding: 9px 8px 5px 3px;
  anchor-name: --foo;
}
.b {
  background: blue;
  position: absolute;
  width: 50px;
  height: 100px;
  border: 5px solid lime;
  padding: 15px;
  position-anchor: --foo;
  position-area: center bottom
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
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
      "message": "Unknown property “position-area”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
