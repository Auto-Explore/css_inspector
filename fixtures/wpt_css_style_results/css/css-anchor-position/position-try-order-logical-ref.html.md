# css/css-anchor-position/position-try-order-logical-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-order-logical-ref.html"
}
```

## style[0]

```css

.container {
  position: relative;
  display: inline-block;
  vertical-align: middle;
  width: 100px;
  height: 100px;
  margin: 10px;
  border: solid 3px;
}

.anchor {
  position: absolute;
  anchor-name: --a;
  width: 25px;
  height: 25px;
  left: 50px;
  top: 25px;
  background: dodgerblue;
}

.anchored {
  position: absolute;
  position-anchor: --a;
  position-try-fallbacks: flip-block flip-inline;
  width: 15px;
  height: 15px;
  background: green;
}
```

```json
{
  "errors": 4,
  "messages": [
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
