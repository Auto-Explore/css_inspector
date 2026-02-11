# css/css-layout-api/at-supports-rule.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/at-supports-rule.https.html"
}
```

## style[0]

```css

#test {
  content: 'fail';
}

@supports (display: layout(foo)) {
  #test {
    content: 'pass';
  }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
