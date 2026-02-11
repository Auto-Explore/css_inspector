# css/css-conditional/container-queries/crashtests/chrome-custom-highlight-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/crashtests/chrome-custom-highlight-crash.html"
}
```

## style[0]

```css

  @container (width) {
    section::highlight(custom-highlight) {
      --foo: bar;
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
