# css/css-conditional/container-queries/var-evaluation.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/var-evaluation.html"
}
```

## style[0]

```css

  #container {
    container-type: size;
    width: 200px;
    --query: 100px;
    --invalid-value: initial;
  }

  @container (width > var(--invalid-value, var(--query, 500px))) {
    #target {
      height: 50px;
      background-color: green;
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
