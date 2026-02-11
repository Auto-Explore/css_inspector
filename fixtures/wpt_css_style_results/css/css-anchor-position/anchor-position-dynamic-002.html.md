# css/css-anchor-position/anchor-position-dynamic-002.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-dynamic-002.html"
}
```

## style[0]

```css

#container {
  position: relative;
}
#anchor1 {
  anchor-name: --a1;
}
#anchor2 {
  anchor-name: --a2;
}
#anchor1, #anchor2 {
  width: 5px;
  height: 7px;
  background: orange;
}
.after #anchor1, .after #anchor2 {
  width: 10px;
}
.target {
  position: absolute;
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
      "message": "Unknown property “anchor-name”.",
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
