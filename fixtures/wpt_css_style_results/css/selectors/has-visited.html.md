# css/selectors/has-visited.html

```json
{
  "format_version": 3,
  "file": "css/selectors/has-visited.html"
}
```

## style[0]

```css

  #parent1:has(:link) {
    color: green;
  }
  #parent2:has(:visited) {
    color: yellow;
  }
  #parent3:has(:any-link) {
    color: yellowgreen;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
