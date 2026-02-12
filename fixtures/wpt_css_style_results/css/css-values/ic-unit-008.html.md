# css/css-values/ic-unit-008.html

```json
{
  "format_version": 3,
  "file": "css/css-values/ic-unit-008.html"
}
```

## style[0]

```css

  @font-face
    {
      font-family: IcTestFullWidth;
      src: url(resources/IcTestFullWidth.woff2);
    }

  div
    {
      float: left;
      font-family: IcTestFullWidth;
      font-size: 80px; /* arbitrary font size */
    }

  div#test-blue
    {
      background-color: blue;
      height: 1.8em;
      width: 5ic;
    }

  div#reference-orange
    {
      background-color: orange;
      clear: left;
      color: orange;
      line-height: 1.8; /* arbitrary line-height */
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
