# css/css-writing-modes/reference/text-shadow-orientation-upright-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/reference/text-shadow-orientation-upright-001-ref.html"
}
```

## style[0]

```css

  div
    {
      color: transparent;
      font-family: Ahem;
      font-size: 100px;
      line-height: 1;
    }

  div#purple
    {
      text-shadow: 1em 0em purple;
    }

  div#orange-blue
    {
      color: yellow;
      margin-left: 1em;
      text-shadow: -1em 0em orange, 1em 0em blue;
    }

  div#fuchsia
    {
     text-shadow: 1em 0em fuchsia;
    }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
