# css/css-backgrounds/background-attachment-fixed-inside-transform-1.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-attachment-fixed-inside-transform-1.html"
}
```

## style[0]

```css

      body {
        height: 4000px;
        margin: 0;
      }

      #outer {
        margin: 200px;
        height: 700px;
        width: 300px;
        transform: rotate(45deg);
        overflow: hidden;
      }

      #inner {
        height: 700px;
        background-image: radial-gradient(farthest-corner at center, blue, black);
        background-size: 300px 300px;
        background-position: 200px 200px;
        background-color: lime;
        background-repeat: no-repeat;
        background-attachment: fixed;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
