# css/css-conditional/container-queries/container-name-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-name-invalidation.html"
}
```

## style[0]

```css

  div {
    color: black;
  }
  #outer {
    container-name: c1;
    container-type: inline-size;
    width: 300px;
  }

  #inner {
    container-name: c2;
    container-type: inline-size;
    width: 200px;
  }

  #intermediate {
    width: 250px;
  }

  @container c1 (width: 250px) {
    #child {
      color: green;
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
