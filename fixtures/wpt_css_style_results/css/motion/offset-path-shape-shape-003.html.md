# css/motion/offset-path-shape-shape-003.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-shape-shape-003.html"
}
```

## style[0]

```css

#outer {
  width: 600px;
  height: 400px;
  border: 50px solid transparent;
}
#box {
  width: 100px;
  height: 100px;
  background-color: green;
  offset-path: shape(from 50px calc(-10% + 90px),
                     hline by 100%,
                     vline to calc(100% + 50px))
               content-box;
  offset-distance: 80%;
  border-radius: 50% 50% 0 0;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
