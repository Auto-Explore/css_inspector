# css/css-flexbox/flexbox-items-as-stacking-contexts-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-items-as-stacking-contexts-003-ref.html"
}
```

## style[0]

```css

    .flexContainer {
      background: orange;
      width: 70px;
      height: 20px;
      padding: 2px;
      margin-bottom: 2px;
    }
    .item1 {
      display: inline-block;
      background: lightblue;
      width: 30px;
      height: 16px;
      padding: 2px;
      margin-right: 2px;
      vertical-align: top;
    }
    .item2 {
      display: inline-block;
      background: yellow;
      width: 30px;
      height: 16px;
      padding: 2px;
      vertical-align: top;
    }
    .grandchildA {
      background: purple;
      width: 80px;
      height: 6px;
      position: relative;
      z-index: 10;
    }
    .grandchildB {
      background: teal;
      width: 80px;
      height: 6px;
      position: relative;
      z-index: 20;
    }
    .grandchildC {
      background: lime;
      width: 20px;
      height: 16px;
      position: relative;
      /* This z-index should interleave this content
         between grandchildA and grandchildB: */
      z-index: 15;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
