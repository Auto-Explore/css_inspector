# css/css-contain/contain-size-064.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-064.html"
}
```

## style[0]

```css

div { font-size: 50px; }
.red { background: red; }
.green { background: green; }
.grid {
    display: grid;
    grid-template-columns: min-content max-content;
}
#test { contain: size; }
#test::after { content: "\a0" ; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
