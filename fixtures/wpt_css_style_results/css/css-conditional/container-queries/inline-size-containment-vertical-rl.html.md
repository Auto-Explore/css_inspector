# css/css-conditional/container-queries/inline-size-containment-vertical-rl.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/inline-size-containment-vertical-rl.html"
}
```

## style[0]

```css

  #ancestry { writing-mode: vertical-rl; }
  #keg { container-type: inline-size; }
  @container (max-height: 200px) {
    #target { width: 400px; }
  }
  @container (min-height: 400px) {
    #target { width: 20px; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
