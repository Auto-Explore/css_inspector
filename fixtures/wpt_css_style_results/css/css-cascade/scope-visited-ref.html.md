# css/css-cascade/scope-visited-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-visited-ref.html"
}
```

## style[0]

```css

    /* The visited background-color magically gets the alpha of the
     unvisited color, which by default is rgba(0, 0, 0, 0). Set alpha to
     255 so that visited colors also gets this alpha. */
  * { background-color: rgba(255, 255, 255, 255); }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
