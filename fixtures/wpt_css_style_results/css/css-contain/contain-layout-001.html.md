# css/css-contain/contain-layout-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-001.html"
}
```

## style[0]

```css

#pa {
  contain: layout;
  height: 100vh; /*If layout containment applies, the span becomes a BFC, height applies, and knocks SS off the page */
}

#ss {
  vertical-align: bottom;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
