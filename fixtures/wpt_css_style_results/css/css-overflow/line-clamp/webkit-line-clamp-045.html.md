# css/css-overflow/line-clamp/webkit-line-clamp-045.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/webkit-line-clamp-045.html"
}
```

## style[0]

```css

.clamp {
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 4;
  font: 16px / 32px serif;
  white-space: pre;
  background-color: yellow;
  padding: 0 4px;
  overflow: hidden; /* can be removed once implementations update their old -webkit-line-clamp implementations */

  /* If display: -webkit-box behaves the same as without -webkit-line-clamp,
   * these properties would cause the anonymous inline box to be centered. */
  -webkit-box-align: center;
  -webkit-box-pack: center;
  align-items: center;
  justify-content: center;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “-webkit-box-orient”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-line-clamp”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-box-align”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-box-pack”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
