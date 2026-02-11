# css/css-conditional/container-queries/transition-style-change-event.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/transition-style-change-event.html"
}
```

## style[0]

```css

  .container { container-type: size }
  #outer {
    width: 100px;
    color: green;
  }
  @container (min-width: 200px) {
    #inner { color: red }
  }
  @container (min-width: 400px) {
    #target {
      color: green;
      transition: color 1s step-start;
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
