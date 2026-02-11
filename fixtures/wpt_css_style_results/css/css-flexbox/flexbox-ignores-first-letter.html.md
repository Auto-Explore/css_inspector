# css/css-flexbox/flexbox-ignores-first-letter.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-ignores-first-letter.html"
}
```

## style[0]

```css

  body { line-height: 20px; }
  .flexbox { display: flex; }
  .inline-flexbox { display: inline-flex; }
  .flexbox-first-letter::first-letter { line-height: 100px; color: red; }
  .container-first-letter::first-letter { line-height: 200px; color: green; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
