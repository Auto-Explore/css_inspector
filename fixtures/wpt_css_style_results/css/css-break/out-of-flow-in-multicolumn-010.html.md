# css/css-break/out-of-flow-in-multicolumn-010.html

```json
{
  "format_version": 3,
  "file": "css/css-break/out-of-flow-in-multicolumn-010.html"
}
```

## style[0]

```css

  #multicol {
    column-count: 3;
    column-fill: auto;
    column-gap: 0px;
    /* last child is a spanner that takes up 33.33% so available height for
       .rel and .abs is 100px */
    height: 150px;
    width: 150px;
    margin-left: -50px;
  }
  .rel {
    position: relative;
  }
  .abs {
    position: absolute;
    /* offset one full 100px column (accounted for by negative margin on #multicol */
    top: 100px;
    width: 50px;
    height: 66.667%;
    background: green;
  }
  .spanner {
    column-span: all;
    height: 33.33%;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
