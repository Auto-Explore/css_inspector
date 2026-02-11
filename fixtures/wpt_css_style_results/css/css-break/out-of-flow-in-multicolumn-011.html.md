# css/css-break/out-of-flow-in-multicolumn-011.html

```json
{
  "format_version": 3,
  "file": "css/css-break/out-of-flow-in-multicolumn-011.html"
}
```

## style[0]

```css

  #multicol {
    /* Set things up so that the absolutely positioned element takes up two
       columns of 50px each. Start in the second column so offset by negative
       50px margin. */
    columns: 5;
    width: 250px;
    height: 100px;
    margin-left: -50px;
    column-fill: auto;
    column-gap: 0px;
  }
  .rel {
    position: relative;
  }
  .abs {
    position: absolute;
    top: 0px;
    width: 50px;
    height: 100%;
    background: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
