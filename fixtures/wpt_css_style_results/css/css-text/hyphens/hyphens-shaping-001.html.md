# css/css-text/hyphens/hyphens-shaping-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text/hyphens/hyphens-shaping-001.html"
}
```

## style[0]

```css

  @font-face {
    font-family: 'csstest_noto';
    src: url('/fonts/noto/NotoNaskhArabic-regular.woff2') format('woff2');
  }
  div {
    font-family: 'csstest_noto';
    font-size: 3em;
    line-height: 1; /* Not strictly needed, but it gets quite tall otherwise, so this helps fit the screen. */
    width: 0;
    hyphens: manual;
    margin: 1em auto;
  }
  .ref { color: orange; }
  span { color: transparent; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
