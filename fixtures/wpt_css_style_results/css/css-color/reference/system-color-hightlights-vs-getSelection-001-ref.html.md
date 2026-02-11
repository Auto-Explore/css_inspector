# css/css-color/reference/system-color-hightlights-vs-getSelection-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-color/reference/system-color-hightlights-vs-getSelection-001-ref.html"
}
```

## style[0]

```css

  div
    {
      font-size: 40px;
      line-height: 1;
    }

  /*
     https://www.w3.org/TR/css-pseudo-4/#highlight-bounds
     For text, the corresponding overlay must cover at least
     the entire em box and may extend further above/below the
     em box to the line box edges.
  */

  span
    {
      background-color: Highlight;
      color: HighlightText;
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
