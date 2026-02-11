# css/css-sizing/contain-intrinsic-size/auto-018.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/auto-018.html"
}
```

## style[0]

```css

#target {
  width: max-content;
  height: max-content;
}
.cis-auto {
  contain-intrinsic-height: auto 2px;
}
.skip-contents {
  content-visibility: hidden;
}
.size-100-50 {
  inline-size: 100px;
  block-size: 50px;
}
.size-75-25 {
  inline-size: 75px;
  block-size: 25px;
}
.vertical {
  writing-mode: vertical-lr;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “contain-intrinsic-height”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
