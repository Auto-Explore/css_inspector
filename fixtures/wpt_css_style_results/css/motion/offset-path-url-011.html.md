# css/motion/offset-path-url-011.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-url-011.html"
}
```

## style[0]

```css

      #outer {
        position: relative;
        left: 50px;
        top: 50px;
        width: 300px;
        height: 100px;
      }
      #target {
        width: 50px;
        height: 50px;
        background-color: green;
        /* This behaves as 'offset-path: path("m 0 0")' */
        offset-path: url(#outer);
        offset-anchor: 50% 50%;
        offset-distance: 0%;
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
