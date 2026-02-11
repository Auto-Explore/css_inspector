# css/css-anchor-position/position-visibility-no-overflow-nested.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-no-overflow-nested.html"
}
```

## style[0]

```css

.container {
    position: relative;
    width: 100px;
    height: 100px;
    background: green;
}
.outer {
    position: absolute;
    top: 10px;
    width: 100px;
    height: 100px;
    background: yellow;
    position-visibility: no-overflow;
}
.inner {
    position: absolute;
    width: 50px;
    height: 50px;
    background: red;
    position-visibility: no-overflow;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “position-visibility”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-visibility”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
