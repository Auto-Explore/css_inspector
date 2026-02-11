# css/css-logical/parsing/border-inline-width-computed.html

```json
{
  "format_version": 3,
  "file": "css/css-logical/parsing/border-inline-width-computed.html"
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
    font-size: 40px;
    border-inline-style: dotted; /* Avoid border-inline-*-width computed style 0 */
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
