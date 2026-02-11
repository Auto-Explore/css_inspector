# css/css-conditional/container-queries/style-query-guaranteed-invalid.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/style-query-guaranteed-invalid.tentative.html"
}
```

## style[0]

```css

  #target { background-color: green; }
  @container --root style(--prop: var(--unknown)) {
    #target { background-color: red; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
