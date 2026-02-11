# css/css-pseudo/marker-text-emphasis-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-text-emphasis-ref.html"
}
```

## style[0]

```css

ol {
  float: left;
  width: 50px;
  list-style: none;
  line-height: 2;
}
.text-emphasis-shorthand {
  -webkit-text-emphasis: circle green;
  text-emphasis: circle green;
}
.text-emphasis-longhands {
  -webkit-text-emphasis-style: circle;
  -webkit-text-emphasis-color: green;
  -webkit-text-emphasis-position: under right;
  text-emphasis-style: circle;
  text-emphasis-color: green;
  text-emphasis-position: under right;
}
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unknown property “-webkit-text-emphasis”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-emphasis”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-text-emphasis-style”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-text-emphasis-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-text-emphasis-position”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-emphasis-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
