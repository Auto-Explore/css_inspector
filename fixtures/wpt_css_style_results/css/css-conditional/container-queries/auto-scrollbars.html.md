# css/css-conditional/container-queries/auto-scrollbars.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/auto-scrollbars.html"
}
```

## style[0]

```css

  #scroller {
    height: 100px;
    width: 100px;
    overflow-y: auto;
  }
  #container {
    container-type: inline-size;
  }
  #inner {
    height: 100px;
    border-bottom: 1px solid red;
  }
  @container (max-width: 99px) {
    #inner {
      height: 50px;
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
