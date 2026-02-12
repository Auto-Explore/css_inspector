# css/css-flexbox/flexbox-overflow-vert-002.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-overflow-vert-002.html"
}
```

## style[0]

```css

    .flexContainer {
      background: purple;
      display: flex;
      flex-direction: column;
      align-items: center;
      width: 70px;
      height: 70px;
      margin-bottom: 5px;
    }
    .bigItem {
      background: blue;
      width: 10px;
      /* Tall border (taller than our container): */
      border: solid coral;
      border-width: 2px 50px;
      flex: 3;
    }
    .smallItem {
      background: teal;
      width: 20px;
      flex: 1;
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
