# css/cssom/selectorText-modification-restyle-002.html

```json
{
  "format_version": 3,
  "file": "css/cssom/selectorText-modification-restyle-002.html"
}
```

## style[0]

```css

  #container { color: red }
  .subtree * { color: pink }
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

## style[1]

```css
nomatch { color: green }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
