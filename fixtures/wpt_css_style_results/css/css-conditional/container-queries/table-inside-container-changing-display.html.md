# css/css-conditional/container-queries/table-inside-container-changing-display.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/table-inside-container-changing-display.html"
}
```

## style[0]

```css

  @supports not (container-type: inline-size) {
    #container { display: none !important; }
  }
  #container {
    width: 200px;
    height: 200px;
    container-type: inline-size;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
