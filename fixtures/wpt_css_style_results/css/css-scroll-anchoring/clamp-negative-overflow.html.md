# css/css-scroll-anchoring/clamp-negative-overflow.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-anchoring/clamp-negative-overflow.html"
}
```

## style[0]

```css

        #scroller {
            overflow: scroll;
            width: 500px;
            height: 500px;
        }
        #anchor {
            position: relative;
            width: 100px;
            height: 100px;
            margin-top: 100px;
            margin-bottom: 1000px;
            background-color: blue;
        }
        #positioned {
            position: absolute;
            width: 10px;
            height: 10px;
            top: -200px;
            background-color: yellow;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
