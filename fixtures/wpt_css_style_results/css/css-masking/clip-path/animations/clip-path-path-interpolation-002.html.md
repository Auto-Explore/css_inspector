# css/css-masking/clip-path/animations/clip-path-path-interpolation-002.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-path-interpolation-002.html"
}
```

## style[0]

```css

    @keyframes anim {
      from {
        clip-path: path(evenodd, 'M20,20h60 v60 h-60z M30,30 h40 v40 h-40z');
      }
      to {
        clip-path: path(evenodd, 'M50,50h50 v50 h-50z M20,20 h50 v50 h-50z');
      }
    }
    #rect {
      width: 100px;
      height: 100px;
      background-color: green;
      animation: anim 10s -5s paused linear;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
