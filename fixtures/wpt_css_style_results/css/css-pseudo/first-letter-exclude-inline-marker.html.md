# css/css-pseudo/first-letter-exclude-inline-marker.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/first-letter-exclude-inline-marker.html"
}
```

## style[0]

```css

body { margin-left: 5em; }
ibi::first-letter, ibo::first-letter { color: green }
li:first-child > * { list-style-type: lower-alpha; list-style-type: 'marker ' }
li:last-child > ::marker { content: 'marker ' }
ibi { display:inline flow-root list-item; list-style-position: inside; }
ibo { display:inline flow-root list-item; list-style-position: outside; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
