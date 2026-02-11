# css/css-conditional/container-queries/viewport-units.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/viewport-units.html"
}
```

## style[0]

```css

  #vw { container-type: inline-size; width: 10vw; }
  #vh { container-type: inline-size; width: 10vh; }

  @container (min-width: 10vw) {
    #vw span { color: green }
  }
  @container (min-width: 11vw) {
    #vw span { color: red }
  }
  @container (min-width: 10vh) {
    #vh span { color: green }
  }
  @container (min-width: 11vh) {
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
