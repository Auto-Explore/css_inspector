# css/css-images/gradient/gradient-single-stop-none-interpolation-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-single-stop-none-interpolation-ref.html"
}
```

## style[0]

```css

    div {
      height: 100px;
    }
    #basic {
      /* "none" should resolve to zero when we only have one single stop. */
      background: linear-gradient(to right in srgb, color(srgb 0 0.5 0.5));
    }
    #multipleNone {
      /* "none" and "none" gives zero. */
      background: linear-gradient(to right in srgb, color(srgb 0 0 0));
    }
    #allNone {
      /* "none" and "none" gives zero. */
      background: linear-gradient(to right in srgb, color(srgb 0 0 0));
    }
    #noneHue {
      background: linear-gradient(to right in oklch, oklch(0.8 0.4 0));
    }
    #noneHueLonger {
      background: linear-gradient(to right in oklch longer hue, oklch(0.5 0.3 0));
    }
 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
