# css/css-conditional/container-queries/size-container-writing-mode-change.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/size-container-writing-mode-change.html"
}
```

## style[0]

```css

  #container {
    container-type: size;
    width: 500px;
    height: 300px;
  }
  #target {
    @container (inline-size = 300px) {
      color: green;
    }
    @container (inline-size = 500px) {
      color: red;
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
