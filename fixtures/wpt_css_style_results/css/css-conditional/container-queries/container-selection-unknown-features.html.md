# css/css-conditional/container-queries/container-selection-unknown-features.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-selection-unknown-features.html"
}
```

## style[0]

```css

  #container {
    container-type: size;
    width: 200px;
    --foo: bar;
  }
  @container (width > 0px) {
    div { color: green }
  }
  @container (width > 0px) or (foo: bar) {
    #t1 { color: red }
  }
  @container (width > 0px) or foo(bar) {
    #t2 { color: red }
  }
  @container style(--foo: bar) or (foo: bar) {
    #t3 { color: red }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
