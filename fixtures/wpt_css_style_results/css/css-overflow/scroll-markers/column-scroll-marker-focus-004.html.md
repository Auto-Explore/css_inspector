# css/css-overflow/scroll-markers/column-scroll-marker-focus-004.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/column-scroll-marker-focus-004.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }
  #scrollable {
    scroll-marker-group: after;
    overflow: hidden;
    height: 100px;
    line-height: 20px;
  }
  #multicol {
    columns: 11;
    column-gap: 10px;
    column-fill: auto;
    column-rule: solid;
    height: 100%;
    orphans: 1;
    widows: 1;
  }
  #scrollable::scroll-marker-group {
    display: flex;
    height: 20px;
    background: hotpink;
  }
  #multicol::column::scroll-marker {
    content: "";
    width: 20px;
    height: 20px;
    margin-right: 5px;
    background: blue;
  }
  #multicol::column::scroll-marker:focus {
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
