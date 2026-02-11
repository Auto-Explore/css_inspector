# css/css-pseudo/first-letter-skip-marker.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/first-letter-skip-marker.html"
}
```

## style[0]

```css

body { margin-left:100px; }
ol > li { list-style-position: inside }
li::first-letter { color: green }
li:first-child { list-style-type: lower-alpha; list-style-type: '::marker ' }
li:last-child::marker { content: '::marker ' }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
