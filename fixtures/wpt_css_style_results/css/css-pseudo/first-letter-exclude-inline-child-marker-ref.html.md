# css/css-pseudo/first-letter-exclude-inline-child-marker-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/first-letter-exclude-inline-child-marker-ref.html"
}
```

## style[0]

```css

body { margin-left: 5em; }
li { list-style-position: inside }
li:first-child > * { list-style-type: 'marker '; }
li:last-child > ::marker { content: 'marker '; }
span { display:inline list-item; }
ibi { display:inline flow-root list-item; list-style-position: inside; }
ibo { display:inline flow-root list-item; list-style-position: outside; }
first-letter { color:green; }
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
