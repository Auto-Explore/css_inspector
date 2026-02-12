# css/css-overflow/scroll-markers/column-scroll-marker-focus-003.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/column-scroll-marker-focus-003.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }
  #container {
    scroll-marker-group: before;
    overflow: scroll;
    columns: 7;
    column-gap: 10px;
    column-fill: auto;
    column-rule: solid;
    height: 100px;
    line-height: 20px;
    orphans: 1;
    widows: 1;
  }
  #container::scroll-marker-group {
    display: flex;
    height: 20px;
    background: hotpink;
  }
  #container::column::scroll-marker {
    content: "";
    width: 20px;
    height: 20px;
    margin-right: 5px;
    background: blue;
  }
  #container::column::scroll-marker:focus {
    background: cyan;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
