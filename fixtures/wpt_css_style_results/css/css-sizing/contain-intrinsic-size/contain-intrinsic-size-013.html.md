# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-013.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-013.html"
}
```

## style[0]

```css

#border {
  width: max-content;
  border: 1px solid black;
}
#border > div {
  contain-intrinsic-size: 55px 66px;
  contain: size;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
