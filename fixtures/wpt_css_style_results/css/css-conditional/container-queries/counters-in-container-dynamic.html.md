# css/css-conditional/container-queries/counters-in-container-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/counters-in-container-dynamic.html"
}
```

## style[0]

```css

  #container {
    container-type: size;
    width: 200px;
    height: 200px;
  }

  #counter::before {
    content: counter(my-counter);
  }

  @container (min-width: 300px) {
    #counter {
      counter-reset: my-counter 100;
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
