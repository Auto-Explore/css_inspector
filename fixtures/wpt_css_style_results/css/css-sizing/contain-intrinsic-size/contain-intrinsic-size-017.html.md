# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-017.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-017.html"
}
```

## style[0]

```css

body {
  writing-mode: vertical-lr;
}
#target {
  contain-intrinsic-size: 100px 200px;
  contain: size;
  inline-size: min-content;
  block-size: auto;
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
