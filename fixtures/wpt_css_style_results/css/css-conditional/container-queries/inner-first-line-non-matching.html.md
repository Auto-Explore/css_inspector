# css/css-conditional/container-queries/inner-first-line-non-matching.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/inner-first-line-non-matching.html"
}
```

## style[0]

```css

  #outer::first-line { color: green }
  @container (width > 99999px) {
    #inner::first-line { color: red }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
