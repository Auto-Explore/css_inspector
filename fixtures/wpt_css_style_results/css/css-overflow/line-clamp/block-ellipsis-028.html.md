# css/css-overflow/line-clamp/block-ellipsis-028.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/block-ellipsis-028.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: 2;
  width: 63.1ch;
  border: 1px solid black;
  font-family: monospace;
  hyphens: manual; /* Initial value, but explicit since that's the target of the test*/
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “line-clamp”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
