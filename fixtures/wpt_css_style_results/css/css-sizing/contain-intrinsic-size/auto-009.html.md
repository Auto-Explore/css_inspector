# css/css-sizing/contain-intrinsic-size/auto-009.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/auto-009.html"
}
```

## style[0]

```css

#target {
  width: max-content;
  height: max-content;
  border: 1px solid;
}
#target::before {
  content: "";
  display: block;
}
.content-100-50::before {
  width: 100px;
  height: 50px;
}
.content-skip {
  content-visibility: hidden;
}
.ciw-auto-2 {
  contain-intrinsic-width: auto 2px;
}
.ciw-auto-20 {
  contain-intrinsic-width: auto 20px;
}
.cih-auto-1 {
  contain-intrinsic-height: auto 1px;
}
.cih-auto-10 {
  contain-intrinsic-height: auto 10px;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “contain-intrinsic-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-height”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-height”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
