# css/css-contain/contain-layout-button-001.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-button-001.tentative.html"
}
```

## style[0]

```css

button {
  border: 5px solid green;
  padding: 0;
  /* We use a nonzero margin-bottom to be sure we're synthesizing a baseline
     from the margin-box rather than from the border-box: */
  margin-bottom: 2px;
  contain: layout;
  color: transparent;
  width: 0;
  height: 0;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
