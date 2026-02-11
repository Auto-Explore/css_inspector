# css/css-conditional/container-queries/multicol-container-001.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/multicol-container-001.html"
}
```

## style[0]

```css

  #multicol {
    container-type: inline-size;
    width: 400px;
    column-count: 2;
    column-gap: 0;
  }
  @container (width = 400px) {
    #first-child { color: green; }
    #second-child { color: green; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
