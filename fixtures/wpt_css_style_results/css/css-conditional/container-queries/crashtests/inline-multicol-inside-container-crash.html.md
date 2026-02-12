# css/css-conditional/container-queries/crashtests/inline-multicol-inside-container-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/crashtests/inline-multicol-inside-container-crash.html"
}
```

## style[0]

```css

  #container {
    container-type: size;
    width: 200px;
    height: 100px;
  }
  @container (width <= 200px) {
    #multicol {
      column-count: 2;
      column-gap: 0;
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
