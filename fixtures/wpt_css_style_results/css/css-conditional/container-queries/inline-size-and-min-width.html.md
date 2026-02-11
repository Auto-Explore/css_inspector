# css/css-conditional/container-queries/inline-size-and-min-width.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/inline-size-and-min-width.html"
}
```

## style[0]

```css

  #container {
    container-type: inline-size;
    min-width: 200px;
    width: fit-content;
  }
  @container (min-width: 200px) {
    #child { color: green }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
