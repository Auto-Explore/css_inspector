# css/css-conditional/container-queries/reattach-container-with-dirty-child.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/reattach-container-with-dirty-child.html"
}
```

## style[0]

```css

  #container {
    container-type: inline-size;
  }
  @container (min-width: 200px) {
    div { color: red }
  }
  @container (max-width: 150px) {
    div { color: lime }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
