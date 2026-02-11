# css/css-backgrounds/parsing/border-width-computed.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/parsing/border-width-computed.html"
}
```

## style[0]

```css

  #box {
    border-style: dotted; /* Avoid border-*-width computed style 0 */
    border-top-width: thin;
    border-right-width: medium;
    border-bottom-width: thick;
  }
  #target {
    border-style: dotted; /* Avoid border-*-width computed style 0 */
    font-size: 40px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
