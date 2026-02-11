# css/css-flexbox/flexbox-flex-flow-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-flex-flow-001-ref.html"
}
```

## style[0]

```css

    .flexContainer {
      height: 60px;
      width: 60px;
      font: 10px sans-serif;
      background: yellow;
      float: left;
      border: 1px solid black;
    }
    .flexContainer > * {
      border: 1px dotted gray;
      width: 28px;
      height: 28px;
      float: left;
    }

    /* The single-line flex containers' flex items are shrunk in main axis: */
    .singleLineHoriz > * {
      width: 13px;
    }
    .singleLineVert  > * {
      height: 13px;
      float: none;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
