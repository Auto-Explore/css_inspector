# css/css-conditional/container-queries/viewport-units-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/viewport-units-dynamic.html"
}
```

## style[0]

```css

  iframe {
    width: 100px;
    height: 100px;
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

    #vw, #vh {
      container-type: inline-size;
      width: 100px;
    }

    @container (min-width: 50vw) {
      #vw span { color: green }
    }
    @container (min-width: 100vw) {
      #vw span { color: red }
    }
    @container (min-width: 50vh) {
      #vh span { color: green }
    }
    @container (min-width: 100vh) {
      #vh span { color: red }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
