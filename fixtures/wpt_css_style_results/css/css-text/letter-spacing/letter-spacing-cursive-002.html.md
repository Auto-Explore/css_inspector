# css/css-text/letter-spacing/letter-spacing-cursive-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text/letter-spacing/letter-spacing-cursive-002.html"
}
```

## style[0]

```css

div {
  margin: 1em;
  font: 24px serif;
}

.letterspace {
  letter-spacing: 1em; /* Does not affect Arabic text, but does affect the space */
}

.ws {
  white-space: pre;
}
.ws::after { /* Fake letter-spacing for the space only */
  content: ' ';
  display: inline-block;
  width: 1em;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
