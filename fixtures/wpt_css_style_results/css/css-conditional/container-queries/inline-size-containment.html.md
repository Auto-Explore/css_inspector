# css/css-conditional/container-queries/inline-size-containment.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/inline-size-containment.html"
}
```

## style[0]

```css

  #keg { container-type: inline-size; }
  @container (max-width: 200px) {
    #target { height: 400px; }
  }
  @container (min-width: 400px) {
    #target { height: 20px; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
