# css/css-overflow/line-clamp/line-clamp-auto-041.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-auto-041.html"
}
```

## style[0]

```css

.clamp {
  font-family: monospace;
  line-clamp: auto;
  max-height: 5lh;
  line-height: 1;
  width: 25.2ch; /* the extra .2 is in case things aren't perfectly monospaced */
}
span {
  font-size: 4em;
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
