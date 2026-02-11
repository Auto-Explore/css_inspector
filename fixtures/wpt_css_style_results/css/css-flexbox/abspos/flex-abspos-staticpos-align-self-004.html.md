# css/css-flexbox/abspos/flex-abspos-staticpos-align-self-004.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/abspos/flex-abspos-staticpos-align-self-004.html"
}
```

## style[0]

```css

    .container {
      display: flex;
      position: relative;
      flex-flow: row-reverse wrap-reverse;
      padding: 1px 2px;
      border: 1px solid black;
      background: yellow;
      margin-bottom: 5px;
      margin-right: 5px;
      float: left; /* For testing in "rows" of containers */
      align-items: center; /* To exercise 'align-self: auto' on children */
    }
    br { clear: both }

    .big > .container {
      height: 10px;
      width: 16px;
    }
    .small > .container {
      height: 2px;
      width: 4px;
    }

    .container > * {
      position: absolute;
      background: teal;
      height: 6px;
      width: 8px;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex-flow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
