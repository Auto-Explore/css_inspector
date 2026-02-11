# css/css-flexbox/flexbox-overflow-horiz-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-overflow-horiz-004-ref.html"
}
```

## style[0]

```css

    .flexContainer {
      background: purple;
      display: flex;
      flex-wrap: wrap;
      width: 70px;
      height: 70px;
      float: left;
      margin-right: 5px;
    }
    .bigItem {
      background: blue;
      width: 50px;
      height: 200px;
    }
    .smallItem {
      background: teal;
      width: 50px;
      height: 20px;
    }
    .hidden > .bigItem {
      /* To match the testcase's "overflow:hidden"-cropped flex item, we
         just use a smaller height that exactly fits the space remaining
         in our container, after wrapping */
      height: 50px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
