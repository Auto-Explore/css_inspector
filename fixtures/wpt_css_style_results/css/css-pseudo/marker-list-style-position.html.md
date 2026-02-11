# css/css-pseudo/marker-list-style-position.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-list-style-position.html"
}
```

## style[0]

```css

body { color: black; background: white; }
#t1 > li { list-style-position: inside; }
#t1 li::marker { list-style-position: outside; color: blue; }

#t2 > li { list-style-position: outside; }
#t2 li::marker { list-style-position: inside; color: blue; }

#t3 > li { list-style-position: inside; }
#t3 > li::first-line { list-style-position: outside; color: blue; }
#t3 li::marker { list-style-position: outside; }

#t4 > li { list-style-position: inside; }
#t4 > li::first-letter { list-style-position: outside; color: blue; }
#t4 li::marker { list-style-position: outside; }

span { font-size: 32pt; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
