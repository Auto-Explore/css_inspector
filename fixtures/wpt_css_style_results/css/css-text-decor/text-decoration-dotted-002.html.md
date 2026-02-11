# css/css-text-decor/text-decoration-dotted-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-decoration-dotted-002.html"
}
```

## style[0]

```css

      div {
        font: 92px Arial, sans-serif;
        -webkit-text-decoration: dotted red underline;
        text-decoration: dotted red underline;
        margin: .5em;
      }
      .test1 {
        text-decoration-thickness:10px;
      }
      .test2 {
        text-decoration-thickness:20px;
      }
      .test3 {
        text-decoration-thickness:30px;
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “-webkit-text-decoration”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
