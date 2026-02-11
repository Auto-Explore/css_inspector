# css/css-flexbox/flexbox-overflow-vert-005.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-overflow-vert-005.html"
}
```

## style[0]

```css

    .flexContainer {
      background: purple;
      display: flex;
      flex-direction: column;
      flex-wrap: wrap;
      align-content: space-around;
      width: 70px;
      height: 70px;
      margin-bottom: 5px;
    }
    .bigItem {
      background: blue;
      flex: none; /* prevent shrinking (so we can intentionally overflow) */
      width: 20px;
      height: 72px;
    }
    .smallItem {
      background: teal;
      width: 20px;
      height: 20px;
    }
    .hidden { overflow: hidden }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
