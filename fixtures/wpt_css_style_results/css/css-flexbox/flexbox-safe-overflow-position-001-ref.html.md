# css/css-flexbox/flexbox-safe-overflow-position-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-safe-overflow-position-001-ref.html"
}
```

## style[0]

```css

    .flex {
      display: flex;
      width: 85px;
      height: 65px;
      border: 1px solid black;
      align-content: flex-end;
      justify-content: center;
      align-items: center;
      float: left;
      clear: both;
      margin-top: 100px;
    }
    .rowNoWrap {
      flex-flow: row nowrap;
    }
    .columnNoWrap {
      flex-flow: column wrap;
    }
    .item {
      border: 1px solid blue;
      background: lightblue;
      width: 28px;
      height: 28px;
      flex-shrink: 0;
    }
    .bigItem {
      border: 1px solid blue;
      background: lightblue;
      width: 28px;
      height: 90px;
      flex-shrink: 0;
    }
    .alignContentStart {
      align-content: start;
    }
    .justifyContentStart {
      justify-content: start;
    }
    .alignSelfStart {
      align-self: start;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
