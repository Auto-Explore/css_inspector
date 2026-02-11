# css/css-flexbox/flexbox-overflow-vert-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-overflow-vert-004-ref.html"
}
```

## style[0]

```css

    .flexContainer {
      background: purple;
      display: flex;
      flex-direction: column;
      flex-wrap: wrap;
      width: 70px;
      height: 70px;
      margin-bottom: 5px;
    }
    .bigItem {
      background: blue;
      width: 200px;
      height: 50px;
    }
    .smallItem {
      background: teal;
      width: 20px;
      height: 50px;
    }
    .hidden > .bigItem {
      /* To match the testcase's "overflow:hidden"-cropped flex item, we
         just use a smaller width that exactly fits the space remaining
         in our container, after wrapping */
      width: 50px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
