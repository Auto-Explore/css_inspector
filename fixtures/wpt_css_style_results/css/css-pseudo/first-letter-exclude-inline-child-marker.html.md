# css/css-pseudo/first-letter-exclude-inline-child-marker.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/first-letter-exclude-inline-child-marker.html"
}
```

## style[0]

```css

body { margin-left: 5em; }
li, span, ibi { list-style-position: inside }
li::first-letter { color: green }
li:first-child > * { list-style-type: 'marker ' }
li:last-child > ::marker { content: 'marker ' }
span { display:inline list-item; }
ibi { display:inline flow-root list-item; }
ibo { display:inline flow-root list-item; list-style-position: outside; }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “display”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “display”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “display”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
