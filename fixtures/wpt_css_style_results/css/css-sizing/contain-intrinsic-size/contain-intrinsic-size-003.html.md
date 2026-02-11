# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-003.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-003.html"
}
```

## style[0]

```css

#target {
  background: lightblue;
  contain-intrinsic-size: 111px 222px;
  contain: size;
  width: 50px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
