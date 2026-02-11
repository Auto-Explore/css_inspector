# css/css-conditional/container-queries/change-display-in-container.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/change-display-in-container.html"
}
```

## style[0]

```css

  .fail { display: inline; }
  .pass { display: none; }
  #container { container-type: size; width: 100px; }
  @container (min-width: 200px) {
    .fail { display: none; }
    .pass { display: inline; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
