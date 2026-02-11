# css/css-conditional/container-queries/container-for-cue.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-for-cue.html"
}
```

## style[0]

```css

    video {
      container-type: size;
      width: 200px;
      height: 200px;
    }
    @container (width = 200px) {
      video::cue { background-color: green }
      video::cue(b) { color: lime }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
