# css/css-flexbox/flexbox-paint-ordering-003.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-paint-ordering-003.html"
}
```

## style[0]

```css

    .container { display: flex; }
    .absPosLowOrder {
      position: absolute;
      order: 5;
      background: red;
      height: 0;
      width: 0;
    }
    .absPosHighOrder {
      position: absolute;
      order: 10;
      height: 0;
      width: 0;
    }
    .redBlock {
      height: 100px;
      width: 100px;
      background: red;
    }
    .limeBlock {
      height: 100px;
      width: 100px;
      background: lime;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
