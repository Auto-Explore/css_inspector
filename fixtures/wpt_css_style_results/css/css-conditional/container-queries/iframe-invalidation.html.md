# css/css-conditional/container-queries/iframe-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/iframe-invalidation.html"
}
```

## style[0]

```css

  iframe {
    width: 200px;
    height: 40px;
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
    @container (min-width: 300px) {
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
