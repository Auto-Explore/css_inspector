# css/css-values/attr-container-style-query.html

```json
{
  "format_version": 3,
  "file": "css/css-values/attr-container-style-query.html"
}
```

## style[0]

```css

  #container {
    --foo: bar;
  }
  @container style(--foo: attr(data-foo type(<custom-ident>))) {
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
