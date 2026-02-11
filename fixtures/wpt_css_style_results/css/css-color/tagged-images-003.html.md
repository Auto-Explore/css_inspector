# css/css-color/tagged-images-003.html

```json
{
  "format_version": 3,
  "file": "css/css-color/tagged-images-003.html"
}
```

## style[0]

```css

    .test { background-color: red; width: 12em; height: 6em; margin-top: 0; }
    .ref { background-color: #090; width: 12em; height: 6em; margin-bottom: 0; } /* red-green swap of #900 sRGB */
    /* solid color #990000 PNG image, iCCP with v2 ICC swapped red-green sRGB profile */
    .test { background: url(./support/swap-990000-iCCP.png); }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
