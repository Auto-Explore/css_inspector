# css/css-anchor-position/reference/anchor-position-circular-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/reference/anchor-position-circular-ref.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
}

#anchored1 {
  position: absolute;
  left: 0;
  top: 0;
  background: orange;
}

#anchored2 {
  position: absolute;
  left: 0;
  top: 100px;
  background: green;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
