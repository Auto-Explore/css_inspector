# css/css-text/text-transform/text-transform-tailoring-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text/text-transform/text-transform-tailoring-002.html"
}
```

## style[0]

```css

.test, .ref { font-size: 36px; font-family: 'Gentium Plus', 'Noto Serif', 'Noto Sans', webfont, sans-serif; border: 1px solid orange; margin: 10px; width: 200px;  padding: 5px; }
/* the CSS above is not part of the test */
.test { text-transform: uppercase; }
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
