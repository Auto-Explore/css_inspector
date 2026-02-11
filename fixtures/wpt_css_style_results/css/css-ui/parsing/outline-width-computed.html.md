# css/css-ui/parsing/outline-width-computed.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/parsing/outline-width-computed.html"
}
```

## style[0]

```css

  #target {
    font-size: 40px;

    border-style: dotted; /* Avoid border-*-width computed style 0 */
    border-top-width: thin;
    border-right-width: medium;
    border-bottom-width: thick;

    outline-style: dotted; /* Avoid outline-width computed style 0 */
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
