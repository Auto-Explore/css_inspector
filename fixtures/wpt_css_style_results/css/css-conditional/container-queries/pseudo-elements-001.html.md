# css/css-conditional/container-queries/pseudo-elements-001.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/pseudo-elements-001.html"
}
```

## style[0]

```css

  :root {
    color: black;
  }

  .container {
    container-type: size;
    width: 200px;
    height: 40px;
  }

  @container (min-width: 300px) {
    #container1 div::before { content: "before"; }
    #container1 div::after { content: "after"; }
    #container2 li::marker { color: green; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
