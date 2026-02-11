# css/css-pseudo/marker-text-decoration-skip-ink.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-text-decoration-skip-ink.html"
}
```

## style[0]

```css

ol {
  float: left;
  width: 50px;
  list-style-position: inside;
  text-decoration: underline;
}
.text-decoration-skip-ink-auto.explicit ::marker,
.text-decoration-skip-ink-auto.inherit {
  text-decoration-skip-ink: auto;
}
.text-decoration-skip-ink-none.explicit ::marker,
.text-decoration-skip-ink-none.inherit {
  text-decoration-skip-ink: none;
}
.marker-alpha {
  list-style-type: lower-alpha;
}
.marker-string {
  list-style-type: "p. ";
}
.marker-content::marker {
  content: "q. ";
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
