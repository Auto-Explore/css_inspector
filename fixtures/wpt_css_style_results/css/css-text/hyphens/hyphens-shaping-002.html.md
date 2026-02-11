# css/css-text/hyphens/hyphens-shaping-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text/hyphens/hyphens-shaping-002.html"
}
```

## style[0]

```css

  @font-face {
    font-family: 'csstest_noto';
    src: url('/fonts/noto/NotoNaskhArabic-regular.woff2') format('woff2');
  }
  /* The main text is invisible,
     but we want to give the UA a chance to use the right hyphen,
     which may be font dependent.
   */
  div {
    font-family: 'csstest_noto';
    font-size: 4em;
    hyphens: manual;
    margin: auto;
    width: 0;
  }
  #test { color: transparent; }
  span { color: black; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
