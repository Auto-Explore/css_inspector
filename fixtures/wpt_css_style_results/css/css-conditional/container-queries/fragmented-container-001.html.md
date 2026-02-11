# css/css-conditional/container-queries/fragmented-container-001.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/fragmented-container-001.html"
}
```

## style[0]

```css

  #multicol {
    width: 400px;
    column-count: 2;
    column-fill: auto;
    column-gap: 0;
    height: 100px;
  }
  #float {
    float: left;
    width: 100px;
    height: 50px;
  }
  #container {
    container-type: inline-size;
    display: flow-root;
    height: 200px;
  }
  #first-child {
    break-after: column;
  }
  @container (width = 100px) {
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
