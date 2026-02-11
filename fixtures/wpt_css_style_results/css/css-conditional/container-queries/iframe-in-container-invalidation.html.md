# css/css-conditional/container-queries/iframe-in-container-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/iframe-in-container-invalidation.html"
}
```

## style[0]

```css

  #container {
    container-type: size;
    width: 200px;
    height: 200px;
  }
  iframe {
    width: 200px;
    height: 40px;
  }
  @container (width > 300px) {
    iframe { width: 400px; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

      div#container {
        container-type: size;
        height: 20px;
      }
      div#child { color: red; }
      @container (width > 300px) {
        div#child { color: green; }
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
