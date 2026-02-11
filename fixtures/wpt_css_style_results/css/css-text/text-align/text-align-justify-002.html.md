# css/css-text/text-align/text-align-justify-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text/text-align/text-align-justify-002.html"
}
```

## style[0]

```css

.test { text-align: justify; direction: ltr; }
/* the CSS below is not part of the test */
.test, .ref { border: 1px solid orange; margin: 20px; width: 450px; color: orange; font: 25px/1 Ahem; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
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
