# css/css-sizing/nested-flexbox-image-percentage-max-height-computes-as-none.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/nested-flexbox-image-percentage-max-height-computes-as-none.html"
}
```

## style[0]

```css

body {
  height: 100px;
}
svg {
  position: relative;
  max-width: 100%;
  max-height: 100%;
  contain: size;
  contain-intrinsic-size: 60px 60px;
  aspect-ratio: 1/1;
}
.outer-flexbox {
  display: flex;
  width: 100%;
  height: 100%;
}
.outer-flexbox-item {
  position: relative;
  min-width: 100%;
}
.inner-flexbox {
  position: absolute;
  display: flex;
  inset: 0px;
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
