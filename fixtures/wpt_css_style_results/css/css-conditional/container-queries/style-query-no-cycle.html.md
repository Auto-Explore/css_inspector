# css/css-conditional/container-queries/style-query-no-cycle.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/style-query-no-cycle.html"
}
```

## style[0]

```css

  #container { --foo: bar; }
  @container style(--foo: var(--foo)) {
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
