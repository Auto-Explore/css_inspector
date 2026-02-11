# css/css-text/overflow-wrap/overflow-wrap-shaping-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text/overflow-wrap/overflow-wrap-shaping-002.html"
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
  }
  section {
    float: left; /* to sizing to the intrinsic min of the ref box*/
    position: relative;
  }
  #ref {
    border: solid orange;
    margin: 1rem;
  }
  #test {
    border: solid blue;
    position: absolute; /* to avoid influencing the size of the section */
    left: 1rem;
    right: 1rem;
    overflow-wrap: anywhere;
  }
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
