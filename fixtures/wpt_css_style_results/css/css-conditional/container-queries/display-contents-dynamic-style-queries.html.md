# css/css-conditional/container-queries/display-contents-dynamic-style-queries.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/display-contents-dynamic-style-queries.html"
}
```

## style[0]

```css

  #container.contents {
    --foo: bar;
    display: contents;
  }
  #target {
    color: red;
  }
  @container style(--foo: bar) {
    #target {
      color: green;
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
