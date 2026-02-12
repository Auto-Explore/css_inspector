# css/css-flexbox/flexbox-collapsed-item-horiz-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-collapsed-item-horiz-001.html"
}
```

## style[0]

```css

    .flexContainer {
      display: flex;
      background: yellow;
      border: 1px dotted black;
      float: left;
      margin: 5px;
    }
    .flexContainer > * {
      /* All flex items have 20px base size */
      width: 20px;
    }
    .collapse {
      visibility: collapse;
    }
    .flexible {
      flex: 1 auto;
    }
    .heightTall {
      height: 40px;
      background: purple;
    }
    .heightAuto {
      background: teal;
    }
    .heightShort {
      height: 10px;
      background: pink;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
