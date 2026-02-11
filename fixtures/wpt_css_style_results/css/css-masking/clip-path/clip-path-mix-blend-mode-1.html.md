# css/css-masking/clip-path/clip-path-mix-blend-mode-1.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-mix-blend-mode-1.html"
}
```

## style[0]

```css

      div {
        position: absolute;
        width: 100px;
        height: 100px;
      }

      div.foreground {
        background-color: rgb(255,0,255);
        clip-path: url(#top_left);
        z-index: 100;
        mix-blend-mode: multiply;
      }

      div.background {
        background-color: rgb(0,255,255);
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
