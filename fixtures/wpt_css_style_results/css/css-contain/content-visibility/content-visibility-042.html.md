# css/css-contain/content-visibility/content-visibility-042.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-042.html"
}
```

## style[0]

```css

#container::before {
  content: "This test ";
  color: green;
}
.hasAfter::after {
  content: "PASSES.";
  background: green;
  color: white;
}
.hidden {
  content-visibility: hidden;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
