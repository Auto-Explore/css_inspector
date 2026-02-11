# css/css-pseudo/marker-text-emphasis.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-text-emphasis.html"
}
```

## style[0]

```css

ol {
  float: left;
  width: 50px;
  list-style-position: inside;
  line-height: 2;
}
.text-emphasis-shorthand.explicit ::marker,
.text-emphasis-shorthand.inherit {
  -webkit-text-emphasis: circle green;
  text-emphasis: circle green;
}
.text-emphasis-longhands.explicit ::marker,
.text-emphasis-longhands.inherit {
  -webkit-text-emphasis-style: circle;
  -webkit-text-emphasis-color: green;
  -webkit-text-emphasis-position: under right;
  text-emphasis-style: circle;
  text-emphasis-color: green;
  text-emphasis-position: under right;
}
.marker-decimal {
  list-style-type: decimal;
}
.marker-string {
  list-style-type: "2. ";
}
.marker-content::marker {
  content: "3. ";
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
