# css/css-flexbox/flexbox-overflow-horiz-002.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-overflow-horiz-002.html"
}
```

## style[0]

```css

    .flexContainer {
      background: purple;
      display: flex;
      align-items: center;
      width: 70px;
      height: 70px;
      float: left;
      margin-right: 5px;
    }
    .bigItem {
      background: blue;
      height: 10px;
      /* Tall border (taller than our container): */
      border: solid coral;
      border-width: 50px 2px;
      flex: 3;
    }
    .smallItem {
      background: teal;
      height: 20px;
      flex: 1;
    }
    .hidden { overflow: hidden }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
