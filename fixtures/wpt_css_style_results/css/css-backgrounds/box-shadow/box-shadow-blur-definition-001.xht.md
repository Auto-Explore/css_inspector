# css/css-backgrounds/box-shadow/box-shadow-blur-definition-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/box-shadow/box-shadow-blur-definition-001.xht"
}
```

## style[0]

```css

    div.stripecontainer {
      width: 600px;
      height: 30px;
      border: medium solid blue;
    }
    div.refimage, div.blurcontainer {
      width: 600px;
      height: 10px;
    }
    div.refimage img {
      display: block;
    }
    div.blurcontainer { position: relative; overflow: hidden }
    div.blur {
      position: absolute;
      width: 2000px;
      height: 2010px;
      top: -1000px; /* places 1000px both above and below container */
      left: -2000px; /* places right edge at left of container */
      box-shadow: 300px 0 100px black;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
