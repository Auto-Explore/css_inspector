# css/css-conditional/at-supports-selector-webkit-slider-thumb.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-selector-webkit-slider-thumb.tentative.html"
}
```

## style[0]

```css

  div {
    background:red;
    height:100px;
    width:100px;
  }
  /* This matches in user agents supporting ::-webkit-slider-thumb. */
  @supports selector(input::-webkit-slider-thumb) {
    div { background-color:green; }
  }
  /* This should never match. */
  @supports selector(input::-webkit-asdf) {
    div { background-color:red; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
