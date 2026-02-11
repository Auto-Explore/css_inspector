# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-018.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-018.html"
}
```

## style[0]

```css

#target {
  background: lightblue;
  width: 55px;
  height: 66px;
  contain-intrinsic-size: 111px 222px;
  contain: size;
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
