# css/css-flexbox/flexbox-overflow-padding-002.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-overflow-padding-002.html"
}
```

## style[0]

```css

  .scroll {
    overflow: auto;
    width: 100px;
    height: 100px;
    background: red;
  }
  .flexContainer {
    display: flex;
    writing-mode: vertical-rl;
    direction: rtl;
    padding: 25px;
    background: green;
  }
  .flexItem {
    flex-shrink: 0;
    width: 30px;
    height: 30px;
    margin: 10px;
  }
  .flexItemOverflow {
    width: 65px;
    height: 65px;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
