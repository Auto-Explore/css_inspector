# css/css-values/reference/ic-unit-009-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-values/reference/ic-unit-009-ref.html"
}
```

## style[0]

```css

  @font-face
    {
      font-family: IcTestFullWidth;
      src: url(/css/css-values/resources/IcTestFullWidth.woff2);
    }

  div
    {
      float: left;
      font-family: IcTestFullWidth;
      font-size: 80px; /* arbitrary font size */
      line-height: 1.8; /* arbitrary line-height */
      writing-mode: vertical-rl;
    }

  div#blue
    {
      background-color: blue;
      color: blue;
    }

  div#orange
    {
      background-color: orange;
      color: orange;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
