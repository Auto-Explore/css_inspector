# css/css-flexbox/flexbox-items-as-stacking-contexts-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-items-as-stacking-contexts-002-ref.html"
}
```

## style[0]

```css

    body { font: 10px sans-serif }
    .flexContainer {
      background: orange;
      width: 70px;
      padding: 2px;
      margin-bottom: 2px;
    }

    .flexContainer > div:first-child {
      margin-right: 10px; /* the space between the flex items, in testcase */
    }

    .item1 {
      display: inline-block;
      background: lightblue;
      width: 30px;
    }
    .item2 {
      display: inline-block;
      background: yellow;
      width: 30px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
