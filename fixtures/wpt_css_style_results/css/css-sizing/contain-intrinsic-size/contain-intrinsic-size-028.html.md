# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-028.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-028.html"
}
```

## style[0]

```css

.test {
  contain: inline-size;
  display: inline-block;
  background: green;
}
.test::before {
  content: '';
  display: block;
  width: 40px;
  height: 20px;
}
.cis-none {
  contain-intrinsic-size: none none;
}
.cis-height {
  contain-intrinsic-size: none 50px;
}
.cis-width {
  contain-intrinsic-size: 100px none;
}
.cis-both {
  contain-intrinsic-size: 100px 50px;
}
.vertical {
  writing-mode: vertical-lr;
}
canvas {
  aspect-ratio: auto;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
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
