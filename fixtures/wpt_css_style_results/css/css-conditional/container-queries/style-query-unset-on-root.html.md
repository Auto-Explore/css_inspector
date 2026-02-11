# css/css-conditional/container-queries/style-query-unset-on-root.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/style-query-unset-on-root.html"
}
```

## style[0]

```css

  :root { container-name: --root; }
  @container --root style(--foo: unset) {
    #target { color: green; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
