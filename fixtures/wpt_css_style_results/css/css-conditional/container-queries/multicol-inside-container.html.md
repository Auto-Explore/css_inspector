# css/css-conditional/container-queries/multicol-inside-container.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/multicol-inside-container.html"
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
  #green {
    display: inline-block;
    width: 100%;
    height: 100px;
    background-color: green;
    vertical-align: bottom;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
