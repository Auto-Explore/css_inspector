# css/css-conditional/container-queries/get-animations.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/get-animations.html"
}
```

## style[0]

```css

  #container {
    container-type: inline-size;
    width: 100px;
  }
  #div { color: red; }
  @keyframes test {
    from { color: green; }
    to { color: green; }
  }
  @container (min-width: 200px) {
    #div { animation: test 1s linear forwards; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
