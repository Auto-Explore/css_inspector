# css/css-conditional/container-queries/style-query-with-unknown-width.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/style-query-with-unknown-width.html"
}
```

## style[0]

```css

  #container {
    container-type: inline-size;
    display: contents;
    --foo: bar;
  }
  @container (width >= 0px) or style(--foo: bar) {
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
