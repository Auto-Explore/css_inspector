# css/css-conditional/container-queries/style-query-document-element.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/style-query-document-element.html"
}
```

## style[0]

```css

  html {
    --foo: bar;
  }
  @container style(--foo: bar) {
     body { color: green; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
