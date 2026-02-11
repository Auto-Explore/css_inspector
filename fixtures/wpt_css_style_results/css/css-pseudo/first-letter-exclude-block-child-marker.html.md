# css/css-pseudo/first-letter-exclude-block-child-marker.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/first-letter-exclude-block-child-marker.html"
}
```

## style[0]

```css

body { margin-left: 5em; font-size: 12px; }
li, o { list-style-position: inside }
li::first-letter { color: green }
li:first-child > * { list-style-type: lower-alpha; list-style-type: 'marker ' }
li:last-child > ::marker { content: 'marker ' }
span, o { display:list-item; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
