# css/css-text/overflow-wrap/reference/overflow-wrap-shaping-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text/overflow-wrap/reference/overflow-wrap-shaping-001-ref.html"
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
    font-size: 4em;
    margin: 1rem;
  }
  section {
    float: left; /* to sizing to the intrinsic min of the ref box*/
  }
  #ref { border: solid orange; }
  #test { border: solid blue; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
