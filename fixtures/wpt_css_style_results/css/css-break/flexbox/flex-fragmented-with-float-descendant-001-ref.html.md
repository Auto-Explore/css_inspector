# css/css-break/flexbox/flex-fragmented-with-float-descendant-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/flex-fragmented-with-float-descendant-001-ref.html"
}
```

## style[0]

```css

    .multicol {
      width: 300px;
      columns: 100px auto;
      max-height: 160px;
      border: 3px solid pink;
    }
    .container {
      display: flex;
    }
    .weird-flex-item {
      border: 4px solid teal;
      outline: 4px solid blue;
    }
    .tallFloat {
      float: left;
      border: 3px solid black;
      height: 500px;
      width: 100px;
      background: yellow;
    }
    .float {
      /* In this reference case, this is not actually a float. */
      background: cyan;
      width: 100px;
    }
    .inside-float {
      height: 30px;
      width: 30px;
      background: purple;
      display: inline-block;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
