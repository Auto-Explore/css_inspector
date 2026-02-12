# css/motion/offset-path-ray-contain-005.html

```json
{
  "format_version": 3,
  "file": "css/motion/offset-path-ray-contain-005.html"
}
```

## style[0]

```css

      #container {
        width: 250px;
        height: 250px;
        /* move container to right 100px to make sure we render the element
           properly */
        transform: translate(100px);
      }
      #target {
        position: relative;
        left: 50px;
        top: 50px;
        width: 150px;
        height: 25px;
        background-color: lime;
        offset-path: ray(-90deg closest-side contain);
        offset-anchor: 200% -300%;
        offset-rotate: -90deg;
        offset-distance: 50%;
        offset-position: auto;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
