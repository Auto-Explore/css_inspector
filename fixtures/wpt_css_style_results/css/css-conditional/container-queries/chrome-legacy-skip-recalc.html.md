# css/css-conditional/container-queries/chrome-legacy-skip-recalc.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/chrome-legacy-skip-recalc.html"
}
```

## style[0]

```css

  #container {
    container-type: inline-size;
  }
  #multicol {
    column-count: 1;
  }

  @supports not (container-type: inline-size) {
    #container { display: none }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
