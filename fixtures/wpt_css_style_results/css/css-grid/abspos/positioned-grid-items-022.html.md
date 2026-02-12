# css/css-grid/abspos/positioned-grid-items-022.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/positioned-grid-items-022.html"
}
```

## style[0]

```css

  #grid {
    display: grid;
    width: 100px;
    height: 100px;
    background-color: blue;
    position: relative;
    padding: 5px;
  }

  .abspos {
    position: absolute;
    top: 0px; bottom: 0px; left: 0px; right: 0px;
    width: 100%;
    height: 100%;
    background-color: hotpink;
  }

  #start {
    grid-area: 1/1/auto/auto;
  }

  #end {
    grid-area: auto/auto/1/1;
  }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
