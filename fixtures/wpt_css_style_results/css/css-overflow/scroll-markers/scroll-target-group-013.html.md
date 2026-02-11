# css/css-overflow/scroll-markers/scroll-target-group-013.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-target-group-013.html"
}
```

## style[0]

```css

  #wrapper {
    position: fixed;
    left: 0;
    top: 0;
  }

  .stg {
    scroll-target-group: auto;
  }

  #scroller {
    overflow: auto;
    height: 130px;
    width: 100px;
  }

  .item {
    width: 100px;
    height: 300px;
    background-color: blue;
    margin: 5px;
  }

  a {
    color: red;
  }

  a:target-current {
    color: green;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “scroll-target-group”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
