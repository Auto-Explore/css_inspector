# css/css-scroll-anchoring/abspos-containing-block-outside-scroller.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-anchoring/abspos-containing-block-outside-scroller.html"
}
```

## style[0]

```css


body { margin: 0; }
#scroller { overflow: scroll; width: 500px; height: 400px; }
#space { height: 1000px; }
#abs {
  position: absolute; background-color: red;
  width: 100px; height: 100px;
  left: 25px; top: 25px;
}
#rel {
  position: relative; background-color: green;
  left: 50px; top: 100px; width: 100px; height: 75px;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
