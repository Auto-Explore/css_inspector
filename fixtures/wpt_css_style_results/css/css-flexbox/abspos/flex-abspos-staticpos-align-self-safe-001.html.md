# css/css-flexbox/abspos/flex-abspos-staticpos-align-self-safe-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/abspos/flex-abspos-staticpos-align-self-safe-001.html"
}
```

## style[0]

```css

    .flex {
      display: flex;
      height: 50px;
      width: 50px;
      border: 3px solid black;
      vertical-align: top;
      margin: 20px;
      position: relative;
    }
    .child {
      border: 2px dotted purple;
      background: teal;
      width: 65px;
      height: 65px;
      align-self: safe center;
      position: absolute;
      background: yellow;
    }
    .rowDir {
      flex-direction: row;
    }
    .colDir {
      flex-direction: column;
    }
    .vertRL {
      writing-mode: vertical-rl;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
