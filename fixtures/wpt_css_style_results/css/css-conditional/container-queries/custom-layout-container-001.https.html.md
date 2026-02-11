# css/css-conditional/container-queries/custom-layout-container-001.https.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/custom-layout-container-001.https.html"
}
```

## style[0]

```css

  #test1 {
    width: 400px;
    height: 100px;
  }
  #outer {
    display: inline; /* Shouldn't pass without layout api support */
    display: layout(half);
    height: 100px;
    container-type: inline-size;
  }
  @container (width = 400px) {
    #inner {
      display: inline; /* Shouldn't pass without layout api support */
      display: layout(half);
      height: 100px;
      container-type: inline-size;
    }
  }
  @container (width = 200px) {
    #green {
      background-color: green;
      height: 100px;
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
