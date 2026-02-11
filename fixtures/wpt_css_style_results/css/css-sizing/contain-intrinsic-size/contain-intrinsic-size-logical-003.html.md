# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-logical-003.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-logical-003.html"
}
```

## style[0]

```css

.test {
  contain: size;
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
  contain-intrinsic-inline-size: none;
  contain-intrinsic-block-size: none;
}
.cis-block {
  contain-intrinsic-inline-size: none;
  contain-intrinsic-block-size: 50px;
}
.cis-inline {
  contain-intrinsic-inline-size: 100px;
  contain-intrinsic-block-size: none;
}
.cis-both {
  contain-intrinsic-inline-size: 100px;
  contain-intrinsic-block-size: 50px;
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
  "errors": 8,
  "messages": [
    {
      "message": "Unknown property “contain-intrinsic-inline-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-block-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-inline-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-block-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-inline-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-block-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-inline-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-block-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
